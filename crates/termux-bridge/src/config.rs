pub const DEFAULT_BIND_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 11557;
pub const SERVER_NAME: &str = "termux-bridge";
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOKEN_FILE_NAME: &str = ".termux_terminal_token";
const TOKEN_LIFETIME_SECONDS: u64 = 180 * 24 * 60 * 60;
const TOKEN_GRACE_SECONDS: u64 = 7 * 24 * 60 * 60;

#[derive(Debug, Clone)]
pub struct Authentication {
    token: String,
    issued_at: u64,
}

impl Authentication {
    pub fn matches(&self, token: Option<&str>) -> bool {
        token.is_some_and(|token| token == self.token)
    }

    pub fn expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(u64::MAX);
        now.saturating_sub(self.issued_at) > TOKEN_LIFETIME_SECONDS + TOKEN_GRACE_SECONDS
    }
}

pub fn load_authentication() -> Result<Authentication, BridgeError> {
    let home = env::var_os("HOME").ok_or_else(|| {
        BridgeError::protocol(
            ErrorCode::InternalError,
            "HOME is required for token lookup",
        )
    })?;
    let path = PathBuf::from(home).join(TOKEN_FILE_NAME);
    let metadata = fs::metadata(&path).map_err(|_| {
        BridgeError::protocol(
            ErrorCode::InternalError,
            format!("Missing bridge token file: {}", path.display()),
        )
    })?;
    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(BridgeError::protocol(
            ErrorCode::InternalError,
            format!("Bridge token file must be mode 0600: {}", path.display()),
        ));
    }
    let contents = fs::read_to_string(&path).map_err(|err| {
        BridgeError::protocol(
            ErrorCode::InternalError,
            format!("Failed to read token file: {err}"),
        )
    })?;
    let mut lines = contents.lines();
    let token = lines.next().unwrap_or_default().trim().to_string();
    let issued_at = lines
        .next()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .ok_or_else(|| {
            BridgeError::protocol(ErrorCode::InternalError, "Token file has no issue time")
        })?;
    if token.is_empty() {
        return Err(BridgeError::protocol(
            ErrorCode::InternalError,
            "Token file has no token",
        ));
    }
    Ok(Authentication { token, issued_at })
}

pub fn websocket_bind_addr() -> String {
    format!("{DEFAULT_BIND_HOST}:{DEFAULT_PORT}")
}

pub fn default_shell() -> &'static str {
    "bash"
}
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{BridgeError, ErrorCode};
