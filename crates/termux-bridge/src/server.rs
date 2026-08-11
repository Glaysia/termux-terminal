use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tracing::{error, info};

use crate::config::{Authentication, load_authentication, websocket_bind_addr};
use crate::error::{BridgeError, ErrorCode};
use crate::protocol::{ClientMessage, OutputStream, ServerMessage};
use crate::pty::{PtyEvent, PtySession};
use crate::session::{SessionEvent, SessionStateMachine};

const WEBSOCKET_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(2);

pub async fn run() -> Result<(), BridgeError> {
    init_tracing();

    let bind_addr = websocket_bind_addr();
    let listener = TcpListener::bind(&bind_addr).await.map_err(|err| {
        BridgeError::protocol(
            ErrorCode::InternalError,
            format!("Failed to bind {bind_addr}: {err}"),
        )
    })?;

    info!(
        "termux-bridge listening on {}",
        listener
            .local_addr()
            .map(|addr| addr.to_string())
            .unwrap_or(bind_addr)
    );

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let _shutdown_tx = shutdown_tx;
    serve_with_authentication(listener, shutdown_rx, Arc::new(load_authentication()?)).await
}

pub async fn serve_with_listener(
    listener: TcpListener,
    shutdown: watch::Receiver<bool>,
) -> Result<(), BridgeError> {
    serve(listener, shutdown, None).await
}

async fn serve_with_authentication(
    listener: TcpListener,
    shutdown: watch::Receiver<bool>,
    authentication: Arc<Authentication>,
) -> Result<(), BridgeError> {
    serve(listener, shutdown, Some(authentication)).await
}

async fn serve(
    listener: TcpListener,
    mut shutdown: watch::Receiver<bool>,
    authentication: Option<Arc<Authentication>>,
) -> Result<(), BridgeError> {
    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let (stream, remote_addr) = accept_result.map_err(|err| {
                    BridgeError::protocol(
                        ErrorCode::InternalError,
                        format!("Failed to accept connection: {err}"),
                    )
                })?;

                let authentication = authentication.clone();
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(stream, authentication).await {
                        error!("connection {remote_addr} failed: {err}");
                    }
                });
            }
            changed = shutdown.changed() => {
                match changed {
                    Ok(()) if *shutdown.borrow() => {
                        info!("shutdown requested");
                        break;
                    }
                    Ok(()) => {}
                    Err(_) => break,
                }
            }
        }
    }

    Ok(())
}

async fn accept_websocket(stream: TcpStream) -> Result<WebSocketStream<TcpStream>, BridgeError> {
    timeout(WEBSOCKET_HANDSHAKE_TIMEOUT, accept_async(stream))
        .await
        .map_err(|_| {
            BridgeError::protocol(ErrorCode::InternalError, "WebSocket handshake timed out")
        })?
        .map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to accept websocket connection: {err}"),
            )
        })
}

async fn handle_connection(
    stream: TcpStream,
    authentication: Option<Arc<Authentication>>,
) -> Result<(), BridgeError> {
    let mut websocket = accept_websocket(stream).await?;

    let mut state = SessionStateMachine::new();
    let mut session: Option<PtySession> = None;

    loop {
        tokio::select! {
            message_result = websocket.next() => {
                let Some(message_result) = message_result else {
                    break;
                };

                let message = message_result.map_err(|err| {
                    BridgeError::protocol(
                        ErrorCode::InternalError,
                        format!("WebSocket receive failed: {err}"),
                    )
                })?;

                match message {
                    Message::Text(text) => {
                        let responses = process_client_text_message(&mut state, &mut session, text.as_ref(), authentication.as_deref()).await;
                        let close_for_auth = responses.iter().any(|response| matches!(response, ServerMessage::Error { code, .. } if code == ErrorCode::AuthenticationFailed.as_str() || code == ErrorCode::TokenExpired.as_str()));
                        for response in responses {
                            send_server_message(&mut websocket, response).await?;
                        }
                        if close_for_auth { break; }
                    }
                    Message::Binary(_) => {
                        websocket
                            .close(Some(CloseFrame {
                                code: CloseCode::Unsupported,
                                reason: "binary frames are not supported in v1".into(),
                            }))
                            .await
                            .map_err(|err| {
                                BridgeError::protocol(
                                    ErrorCode::InternalError,
                                    format!("Failed to close binary websocket connection: {err}"),
                                )
                            })?;
                        break;
                    }
                    Message::Ping(payload) => {
                        websocket
                            .send(Message::Pong(payload))
                            .await
                            .map_err(|err| {
                                BridgeError::protocol(
                                    ErrorCode::InternalError,
                                    format!("Failed to reply to ping: {err}"),
                                )
                            })?;
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => break,
                    Message::Frame(_) => {}
                }
            }
            event = recv_pty_event(&mut session), if session.is_some() => {
                match event {
                    Some(PtyEvent::Output(data)) => {
                        send_server_message(
                            &mut websocket,
                            ServerMessage::TerminalOutput {
                                stream: OutputStream::Pty,
                                data,
                            },
                        ).await?;
                    }
                    Some(PtyEvent::Exit(exit_code)) => {
                        state.clear_session();
                        session = None;
                        send_server_message(
                            &mut websocket,
                            ServerMessage::TerminalExit { exit_code },
                        ).await?;
                    }
                    None => {
                        state.clear_session();
                        session = None;
                    }
                }
            }
        }
    }

    if let Some(session) = session.take() {
        let _ = session.close();
    }

    Ok(())
}

async fn recv_pty_event(session: &mut Option<PtySession>) -> Option<PtyEvent> {
    match session {
        Some(session) => session.next_event().await,
        None => None,
    }
}

async fn process_client_text_message(
    state: &mut SessionStateMachine,
    session: &mut Option<PtySession>,
    text: &str,
    authentication: Option<&Authentication>,
) -> Vec<ServerMessage> {
    let message = match ClientMessage::from_json(text) {
        Ok(message) => message,
        Err(BridgeError::Protocol { code, message }) => {
            return vec![ServerMessage::protocol_error(code, message)];
        }
        Err(_) => {
            return vec![ServerMessage::protocol_error(
                ErrorCode::InternalError,
                "unexpected bridge error while parsing client message",
            )];
        }
    };

    if let (Some(authentication), ClientMessage::Hello { token, .. }) = (authentication, &message) {
        if authentication.expired() {
            return vec![ServerMessage::protocol_error(
                ErrorCode::TokenExpired,
                "The Termux Terminal token expired. Run termux-terminal rotate-token.",
            )];
        }
        if !authentication.matches(token.as_deref()) {
            return vec![ServerMessage::protocol_error(
                ErrorCode::AuthenticationFailed,
                "The Termux Terminal token is invalid.",
            )];
        }
    }

    match state.apply(&message) {
        Ok(Some(SessionEvent::SendHelloAck)) => vec![ServerMessage::hello_ack()],
        Ok(Some(SessionEvent::SessionCreated)) => match PtySession::spawn() {
            Ok(pty_session) => {
                *session = Some(pty_session);
                vec![ServerMessage::SessionReady]
            }
            Err(BridgeError::Protocol { code, message }) => {
                state.clear_session();
                vec![ServerMessage::protocol_error(code, message)]
            }
            Err(_) => {
                state.clear_session();
                vec![ServerMessage::protocol_error(
                    ErrorCode::InternalError,
                    "unexpected bridge error while starting shell session",
                )]
            }
        },
        Ok(Some(SessionEvent::ForwardInput)) => {
            if let (Some(session), ClientMessage::TerminalInput { data }) =
                (session.as_ref(), &message)
            {
                if let Err(BridgeError::Protocol { code, message }) =
                    session.write_input(data.clone())
                {
                    return vec![ServerMessage::protocol_error(code, message)];
                }
                Vec::new()
            } else {
                vec![ServerMessage::protocol_error(
                    ErrorCode::SessionUnavailable,
                    "no active PTY session exists",
                )]
            }
        }
        Ok(Some(SessionEvent::Resize)) => {
            if let (Some(session), ClientMessage::TerminalResize { cols, rows }) =
                (session.as_ref(), &message)
            {
                if let Err(BridgeError::Protocol { code, message }) = session.resize(*cols, *rows) {
                    return vec![ServerMessage::protocol_error(code, message)];
                }
                Vec::new()
            } else {
                vec![ServerMessage::protocol_error(
                    ErrorCode::SessionUnavailable,
                    "no active PTY session exists",
                )]
            }
        }
        Ok(Some(SessionEvent::SessionClosed)) => {
            if let Some(active_session) = session.take() {
                if let Err(BridgeError::Protocol { code, message }) = active_session.close() {
                    state.clear_session();
                    return vec![ServerMessage::protocol_error(code, message)];
                }
            }
            vec![ServerMessage::TerminalExit { exit_code: -1 }]
        }
        Ok(None) => Vec::new(),
        Err(BridgeError::Protocol { code, message }) => {
            vec![ServerMessage::protocol_error(code, message)]
        }
        Err(_) => vec![ServerMessage::protocol_error(
            ErrorCode::InternalError,
            "unexpected bridge error while applying client message",
        )],
    }
}

async fn send_server_message(
    websocket: &mut WebSocketStream<TcpStream>,
    message: ServerMessage,
) -> Result<(), BridgeError> {
    let json = message.to_json()?;
    websocket
        .send(Message::Text(json.into()))
        .await
        .map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to send websocket message: {err}"),
            )
        })?;
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .try_init();
}
