use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidMessage,
    UnsupportedMessageType,
    HandshakeRequired,
    AuthenticationFailed,
    TokenExpired,
    InvalidState,
    SessionExists,
    SessionUnavailable,
    AttachRequired,
    InvalidTerminalSize,
    SpawnFailed,
    InternalError,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidMessage => "INVALID_MESSAGE",
            Self::UnsupportedMessageType => "UNSUPPORTED_MESSAGE_TYPE",
            Self::HandshakeRequired => "HANDSHAKE_REQUIRED",
            Self::AuthenticationFailed => "AUTHENTICATION_FAILED",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::InvalidState => "INVALID_STATE",
            Self::SessionExists => "SESSION_EXISTS",
            Self::SessionUnavailable => "SESSION_UNAVAILABLE",
            Self::AttachRequired => "ATTACH_REQUIRED",
            Self::InvalidTerminalSize => "INVALID_TERMINAL_SIZE",
            Self::SpawnFailed => "SPAWN_FAILED",
            Self::InternalError => "INTERNAL_ERROR",
        }
    }
}

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("{message}")]
    Protocol { code: ErrorCode, message: String },
    #[error("websocket server not implemented yet")]
    ServerNotImplemented,
}

impl BridgeError {
    pub fn protocol(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Protocol {
            code,
            message: message.into(),
        }
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Protocol { code, .. } => *code,
            Self::ServerNotImplemented => ErrorCode::InternalError,
        }
    }
}
