use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::{SERVER_NAME, SERVER_VERSION};
use crate::error::{BridgeError, ErrorCode};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    #[serde(rename = "hello")]
    Hello {
        client: String,
        version: String,
        #[serde(default)]
        token: Option<String>,
    },
    #[serde(rename = "session.create")]
    SessionCreate,
    #[serde(rename = "session.attach")]
    SessionAttach,
    #[serde(rename = "terminal.input")]
    TerminalInput { data: String },
    #[serde(rename = "terminal.resize")]
    TerminalResize { cols: u16, rows: u16 },
    #[serde(rename = "session.close")]
    SessionClose,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "hello.ack")]
    HelloAck { server: String, version: String },
    #[serde(rename = "session.ready")]
    SessionReady,
    #[serde(rename = "terminal.output")]
    TerminalOutput { stream: OutputStream, data: String },
    #[serde(rename = "terminal.exit")]
    TerminalExit {
        #[serde(rename = "exitCode")]
        exit_code: i32,
    },
    #[serde(rename = "error")]
    Error { code: String, message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputStream {
    Pty,
}

impl ClientMessage {
    pub fn from_json(input: &str) -> Result<Self, BridgeError> {
        let value: Value = serde_json::from_str(input).map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InvalidMessage,
                format!("Failed to parse client message: {err}"),
            )
        })?;

        let message_type = value.get("type").and_then(Value::as_str).ok_or_else(|| {
            BridgeError::protocol(ErrorCode::InvalidMessage, "Missing type field")
        })?;

        match message_type {
            "hello" | "session.create" | "session.attach" | "terminal.input"
            | "terminal.resize" | "session.close" => serde_json::from_value(value).map_err(|err| {
                BridgeError::protocol(
                    ErrorCode::InvalidMessage,
                    format!("Failed to decode client message: {err}"),
                )
            }),
            _ => Err(BridgeError::protocol(
                ErrorCode::UnsupportedMessageType,
                format!("Unsupported client message type: {message_type}"),
            )),
        }
    }

    pub fn validate(&self) -> Result<(), BridgeError> {
        match self {
            Self::Hello {
                client, version, ..
            } => {
                if client.trim().is_empty() || version.trim().is_empty() {
                    return Err(BridgeError::protocol(
                        ErrorCode::InvalidMessage,
                        "hello requires non-empty client and version",
                    ));
                }
            }
            Self::TerminalInput { .. } => {}
            Self::TerminalResize { cols, rows } => {
                if *cols == 0 || *rows == 0 {
                    return Err(BridgeError::protocol(
                        ErrorCode::InvalidTerminalSize,
                        "terminal.resize requires cols and rows greater than zero",
                    ));
                }
            }
            Self::SessionCreate | Self::SessionAttach | Self::SessionClose => {}
        }

        Ok(())
    }
}

impl ServerMessage {
    pub fn hello_ack() -> Self {
        Self::HelloAck {
            server: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        }
    }

    pub fn protocol_error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Error {
            code: code.as_str().to_string(),
            message: message.into(),
        }
    }

    pub fn to_json(&self) -> Result<String, BridgeError> {
        serde_json::to_string(self).map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to serialize server message: {err}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hello_message() {
        let message = ClientMessage::from_json(
            r#"{ "type": "hello", "client": "obsidian-plugin", "version": "0.1.0" }"#,
        )
        .unwrap();

        assert_eq!(
            message,
            ClientMessage::Hello {
                client: "obsidian-plugin".to_string(),
                version: "0.1.0".to_string(),
                token: None,
            }
        );
    }

    #[test]
    fn rejects_malformed_json() {
        let err = ClientMessage::from_json(r#"{ "type": "hello""#).unwrap_err();

        assert_eq!(err.code(), ErrorCode::InvalidMessage);
    }

    #[test]
    fn rejects_zero_terminal_size() {
        let err = ClientMessage::TerminalResize { cols: 0, rows: 24 }
            .validate()
            .unwrap_err();

        assert_eq!(err.code(), ErrorCode::InvalidTerminalSize);
    }

    #[test]
    fn rejects_unknown_message_type() {
        let err = ClientMessage::from_json(r#"{ "type": "ping" }"#).unwrap_err();

        assert_eq!(err.code(), ErrorCode::UnsupportedMessageType);
    }

    #[test]
    fn serializes_hello_ack() {
        let json = ServerMessage::hello_ack().to_json().unwrap();

        assert!(json.contains(r#""type":"hello.ack""#));
        assert!(json.contains(r#""server":"termux-bridge""#));
        assert!(json.contains(&format!(r#""version":"{SERVER_VERSION}""#)));
    }

    #[test]
    fn serializes_protocol_error() {
        let json = ServerMessage::protocol_error(ErrorCode::HandshakeRequired, "hello first")
            .to_json()
            .unwrap();

        assert!(json.contains(r#""type":"error""#));
        assert!(json.contains(r#""code":"HANDSHAKE_REQUIRED""#));
        assert!(json.contains(r#""message":"hello first""#));
    }
}
