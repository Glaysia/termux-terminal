use crate::error::{BridgeError, ErrorCode};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    HandshakePending,
    Ready,
    SessionCreated,
    SessionAttached,
}

#[derive(Debug, Default)]
pub struct SessionStateMachine {
    state: ConnectionState,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::HandshakePending
    }
}

impl SessionStateMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn clear_session(&mut self) {
        if matches!(
            self.state,
            ConnectionState::SessionCreated | ConnectionState::SessionAttached
        ) {
            self.state = ConnectionState::Ready;
        }
    }

    pub fn apply(&mut self, message: &ClientMessage) -> Result<Option<SessionEvent>, BridgeError> {
        message.validate()?;

        match (&self.state, message) {
            (ConnectionState::HandshakePending, ClientMessage::Hello { .. }) => {
                self.state = ConnectionState::Ready;
                Ok(Some(SessionEvent::SendHelloAck))
            }
            (ConnectionState::HandshakePending, _) => Err(BridgeError::protocol(
                ErrorCode::HandshakeRequired,
                "hello must be the first client message",
            )),
            (ConnectionState::Ready, ClientMessage::SessionCreate) => {
                self.state = ConnectionState::SessionCreated;
                Ok(Some(SessionEvent::SessionCreated))
            }
            (ConnectionState::Ready, ClientMessage::SessionAttach) => Err(BridgeError::protocol(
                ErrorCode::SessionUnavailable,
                "cannot attach without an active session",
            )),
            (ConnectionState::Ready, ClientMessage::TerminalInput { .. })
            | (ConnectionState::Ready, ClientMessage::TerminalResize { .. }) => {
                Err(BridgeError::protocol(
                    ErrorCode::AttachRequired,
                    "terminal actions require an attached session",
                ))
            }
            (ConnectionState::Ready, ClientMessage::SessionClose) => Err(BridgeError::protocol(
                ErrorCode::SessionUnavailable,
                "cannot close a session that does not exist",
            )),
            (ConnectionState::Ready, ClientMessage::Hello { .. }) => Err(BridgeError::protocol(
                ErrorCode::InvalidState,
                "hello is only valid once per connection",
            )),
            (ConnectionState::SessionCreated, ClientMessage::SessionCreate) => Err(
                BridgeError::protocol(ErrorCode::SessionExists, "a session already exists"),
            ),
            (ConnectionState::SessionCreated, ClientMessage::SessionAttach) => {
                self.state = ConnectionState::SessionAttached;
                Ok(None)
            }
            (ConnectionState::SessionCreated, ClientMessage::SessionClose) => {
                self.state = ConnectionState::Ready;
                Ok(Some(SessionEvent::SessionClosed))
            }
            (ConnectionState::SessionCreated, ClientMessage::TerminalInput { .. })
            | (ConnectionState::SessionCreated, ClientMessage::TerminalResize { .. }) => {
                Err(BridgeError::protocol(
                    ErrorCode::AttachRequired,
                    "terminal actions require an attached session",
                ))
            }
            (ConnectionState::SessionCreated, ClientMessage::Hello { .. }) => {
                Err(BridgeError::protocol(
                    ErrorCode::InvalidState,
                    "hello is only valid once per connection",
                ))
            }
            (ConnectionState::SessionAttached, ClientMessage::TerminalInput { .. }) => {
                Ok(Some(SessionEvent::ForwardInput))
            }
            (ConnectionState::SessionAttached, ClientMessage::TerminalResize { .. }) => {
                Ok(Some(SessionEvent::Resize))
            }
            (ConnectionState::SessionAttached, ClientMessage::SessionClose) => {
                self.state = ConnectionState::Ready;
                Ok(Some(SessionEvent::SessionClosed))
            }
            (ConnectionState::SessionAttached, ClientMessage::SessionCreate) => Err(
                BridgeError::protocol(ErrorCode::SessionExists, "a session already exists"),
            ),
            (ConnectionState::SessionAttached, ClientMessage::SessionAttach) => {
                Err(BridgeError::protocol(
                    ErrorCode::InvalidState,
                    "connection is already attached to the active session",
                ))
            }
            (ConnectionState::SessionAttached, ClientMessage::Hello { .. }) => {
                Err(BridgeError::protocol(
                    ErrorCode::InvalidState,
                    "hello is only valid once per connection",
                ))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionEvent {
    SendHelloAck,
    SessionCreated,
    ForwardInput,
    Resize,
    SessionClosed,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello() -> ClientMessage {
        ClientMessage::Hello {
            client: "obsidian-plugin".to_string(),
            version: "0.1.0".to_string(),
            token: None,
        }
    }

    #[test]
    fn requires_hello_before_session_create() {
        let mut state = SessionStateMachine::new();
        let err = state.apply(&ClientMessage::SessionCreate).unwrap_err();

        assert_eq!(state.state(), ConnectionState::HandshakePending);
        assert_eq!(err.code(), ErrorCode::HandshakeRequired);
    }

    #[test]
    fn hello_moves_state_to_ready() {
        let mut state = SessionStateMachine::new();
        let event = state.apply(&hello()).unwrap();

        assert_eq!(event, Some(SessionEvent::SendHelloAck));
        assert_eq!(state.state(), ConnectionState::Ready);
    }

    #[test]
    fn create_then_attach_reaches_attached_state() {
        let mut state = SessionStateMachine::new();
        state.apply(&hello()).unwrap();
        let create_event = state.apply(&ClientMessage::SessionCreate).unwrap();
        let attach_event = state.apply(&ClientMessage::SessionAttach).unwrap();

        assert_eq!(create_event, Some(SessionEvent::SessionCreated));
        assert_eq!(attach_event, None);
        assert_eq!(state.state(), ConnectionState::SessionAttached);
    }

    #[test]
    fn duplicate_create_is_rejected() {
        let mut state = SessionStateMachine::new();
        state.apply(&hello()).unwrap();
        state.apply(&ClientMessage::SessionCreate).unwrap();
        let err = state.apply(&ClientMessage::SessionCreate).unwrap_err();

        assert_eq!(err.code(), ErrorCode::SessionExists);
    }

    #[test]
    fn input_requires_attach() {
        let mut state = SessionStateMachine::new();
        state.apply(&hello()).unwrap();
        state.apply(&ClientMessage::SessionCreate).unwrap();
        let err = state
            .apply(&ClientMessage::TerminalInput {
                data: "ls\n".to_string(),
            })
            .unwrap_err();

        assert_eq!(err.code(), ErrorCode::AttachRequired);
    }

    #[test]
    fn close_returns_to_ready_state() {
        let mut state = SessionStateMachine::new();
        state.apply(&hello()).unwrap();
        state.apply(&ClientMessage::SessionCreate).unwrap();
        state.apply(&ClientMessage::SessionAttach).unwrap();
        let event = state.apply(&ClientMessage::SessionClose).unwrap();

        assert_eq!(event, Some(SessionEvent::SessionClosed));
        assert_eq!(state.state(), ConnectionState::Ready);
    }

    #[test]
    fn clear_session_resets_active_states_to_ready() {
        let mut state = SessionStateMachine::new();
        state.apply(&hello()).unwrap();
        state.apply(&ClientMessage::SessionCreate).unwrap();
        state.clear_session();
        assert_eq!(state.state(), ConnectionState::Ready);

        state.apply(&ClientMessage::SessionCreate).unwrap();
        state.apply(&ClientMessage::SessionAttach).unwrap();
        state.clear_session();
        assert_eq!(state.state(), ConnectionState::Ready);
    }
}
