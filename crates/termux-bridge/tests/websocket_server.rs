use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use serial_test::serial;
use tokio::net::TcpListener;
use tokio::sync::watch;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use termux_bridge::server;

#[tokio::test]
#[serial]
async fn hello_receives_hello_ack() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    send_hello(&mut socket).await;

    let response = recv_json(&mut socket).await;
    assert_eq!(response["type"], "hello.ack");
}

#[tokio::test]
#[serial]
async fn create_receives_session_ready() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    send_hello(&mut socket).await;
    recv_json(&mut socket).await;
    send_text(&mut socket, r#"{ "type": "session.create" }"#).await;

    let response = recv_json(&mut socket).await;
    assert_eq!(response["type"], "session.ready");
}

#[tokio::test]
#[serial]
async fn attach_and_input_stream_terminal_output() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    establish_attached_session(&mut socket).await;
    send_text(
        &mut socket,
        r#"{ "type": "terminal.input", "data": "printf '__TERMUX_BRIDGE_OK__\\n'\n" }"#,
    )
    .await;

    let response = recv_until(&mut socket, |value| {
        value["type"] == "terminal.output"
            && value["stream"] == "pty"
            && value["data"]
                .as_str()
                .is_some_and(|data| data.contains("__TERMUX_BRIDGE_OK__"))
    })
    .await;

    assert_eq!(response["type"], "terminal.output");
    assert_eq!(response["stream"], "pty");
}

#[tokio::test]
#[serial]
async fn simultaneous_connections_have_independent_shells() {
    let harness = TestServer::spawn().await;
    let mut first_socket = harness.connect().await;
    let mut second_socket = harness.connect().await;

    establish_attached_session(&mut first_socket).await;
    establish_attached_session(&mut second_socket).await;

    send_text(
        &mut first_socket,
        r#"{ "type": "terminal.input", "data": "printf '__TERMUX_BRIDGE_FIRST__\\n'\n" }"#,
    )
    .await;
    send_text(
        &mut second_socket,
        r#"{ "type": "terminal.input", "data": "printf '__TERMUX_BRIDGE_SECOND__\\n'\n" }"#,
    )
    .await;

    let first_output = recv_until(&mut first_socket, |value| {
        value["type"] == "terminal.output"
            && value["data"]
                .as_str()
                .is_some_and(|data| data.contains("__TERMUX_BRIDGE_FIRST__"))
    })
    .await;
    let second_output = recv_until(&mut second_socket, |value| {
        value["type"] == "terminal.output"
            && value["data"]
                .as_str()
                .is_some_and(|data| data.contains("__TERMUX_BRIDGE_SECOND__"))
    })
    .await;

    assert!(
        first_output["data"]
            .as_str()
            .is_some_and(|data| data.contains("__TERMUX_BRIDGE_FIRST__"))
    );
    assert!(
        second_output["data"]
            .as_str()
            .is_some_and(|data| data.contains("__TERMUX_BRIDGE_SECOND__"))
    );
}

#[tokio::test]
#[serial]
async fn resize_after_attach_keeps_session_usable() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    establish_attached_session(&mut socket).await;
    send_text(
        &mut socket,
        r#"{ "type": "terminal.resize", "cols": 100, "rows": 30 }"#,
    )
    .await;
    assert_no_protocol_error(&mut socket).await;

    send_text(
        &mut socket,
        r#"{ "type": "terminal.input", "data": "printf '__TERMUX_BRIDGE_RESIZE__\\n'\n" }"#,
    )
    .await;

    let response = recv_until(&mut socket, |value| {
        value["type"] == "terminal.output"
            && value["data"]
                .as_str()
                .is_some_and(|data| data.contains("__TERMUX_BRIDGE_RESIZE__"))
    })
    .await;

    assert_eq!(response["type"], "terminal.output");
}

#[tokio::test]
#[serial]
async fn session_close_clears_session_and_prevents_attach() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    establish_attached_session(&mut socket).await;

    send_text(&mut socket, r#"{ "type": "session.close" }"#).await;
    recv_until(&mut socket, |value| value["type"] == "terminal.exit").await;

    send_text(&mut socket, r#"{ "type": "session.attach" }"#).await;
    let response = recv_json(&mut socket).await;
    assert_eq!(response["type"], "error");
    assert_eq!(response["code"], "SESSION_UNAVAILABLE");
}

#[tokio::test]
#[serial]
async fn shell_exit_emits_terminal_exit() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    establish_attached_session(&mut socket).await;
    send_text(
        &mut socket,
        r#"{ "type": "terminal.input", "data": "exit\n" }"#,
    )
    .await;

    let response = recv_until(&mut socket, |value| value["type"] == "terminal.exit").await;
    assert_eq!(response["type"], "terminal.exit");
}

#[tokio::test]
#[serial]
async fn create_before_hello_returns_handshake_required() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    send_text(&mut socket, r#"{ "type": "session.create" }"#).await;

    let response = recv_json(&mut socket).await;
    assert_eq!(response["type"], "error");
    assert_eq!(response["code"], "HANDSHAKE_REQUIRED");
}

#[tokio::test]
#[serial]
async fn malformed_json_returns_invalid_message() {
    let harness = TestServer::spawn().await;
    let mut socket = harness.connect().await;

    send_text(&mut socket, r#"{ "type": "hello""#).await;

    let response = recv_json(&mut socket).await;
    assert_eq!(response["type"], "error");
    assert_eq!(response["code"], "INVALID_MESSAGE");
}

#[tokio::test]
#[serial]
async fn disconnect_does_not_preserve_shell_session() {
    let harness = TestServer::spawn().await;

    {
        let mut first_socket = harness.connect().await;
        establish_attached_session(&mut first_socket).await;
    }

    sleep(Duration::from_millis(150)).await;

    let mut second_socket = harness.connect().await;
    send_hello(&mut second_socket).await;
    recv_json(&mut second_socket).await;
    send_text(&mut second_socket, r#"{ "type": "session.attach" }"#).await;

    let response = recv_json(&mut second_socket).await;
    assert_eq!(response["type"], "error");
    assert_eq!(response["code"], "SESSION_UNAVAILABLE");
}

async fn establish_attached_session(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) {
    send_hello(socket).await;
    recv_json(socket).await;
    send_text(socket, r#"{ "type": "session.create" }"#).await;
    recv_json(socket).await;
    send_text(socket, r#"{ "type": "session.attach" }"#).await;
    assert_no_protocol_error(socket).await;
}

async fn send_hello(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) {
    send_text(
        socket,
        r#"{ "type": "hello", "client": "obsidian-plugin", "version": "0.1.0", "token": "test-token" }"#,
    )
    .await;
}

async fn send_text(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    text: &str,
) {
    socket
        .send(Message::Text(text.to_string().into()))
        .await
        .unwrap();
}

async fn recv_json(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let message = timeout(Duration::from_secs(3), socket.next())
        .await
        .expect("timed out waiting for websocket message")
        .expect("websocket closed unexpectedly")
        .expect("websocket read failed");

    match message {
        Message::Text(text) => serde_json::from_str(text.as_ref()).unwrap(),
        other => panic!("expected text websocket message, got {other:?}"),
    }
}

async fn recv_until<F>(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    predicate: F,
) -> Value
where
    F: Fn(&Value) -> bool,
{
    let deadline = tokio::time::Instant::now() + Duration::from_secs(4);
    loop {
        let now = tokio::time::Instant::now();
        let remaining = deadline.saturating_duration_since(now);
        let value = timeout(remaining, recv_json(socket))
            .await
            .expect("timed out waiting for matching websocket message");

        if predicate(&value) {
            return value;
        }
    }
}

async fn assert_no_protocol_error(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) {
    match timeout(Duration::from_millis(150), recv_json(socket)).await {
        Err(_) => {}
        Ok(value) if value["type"] == "terminal.output" => {}
        Ok(value) if value["type"] == "terminal.exit" => {}
        Ok(value) => panic!("expected no immediate protocol error, got {value:?}"),
    }
}

struct TestServer {
    addr: std::net::SocketAddr,
    shutdown_tx: watch::Sender<bool>,
    task: tokio::task::JoinHandle<()>,
}

impl TestServer {
    async fn spawn() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let task = tokio::spawn(async move {
            server::serve_with_listener(listener, shutdown_rx)
                .await
                .expect("server exited with error");
        });

        Self {
            addr,
            shutdown_tx,
            task,
        }
    }

    async fn connect(
        &self,
    ) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>
    {
        let url = format!("ws://{}", self.addr);
        let (socket, _) = connect_async(url).await.unwrap();
        socket
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(true);
        self.task.abort();
    }
}
