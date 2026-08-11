use std::env;
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use tokio::sync::mpsc;

use crate::config::default_shell;
use crate::error::{BridgeError, ErrorCode};

const DEFAULT_COLS: u16 = 80;
const DEFAULT_ROWS: u16 = 24;

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    events_rx: mpsc::UnboundedReceiver<PtyEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyEvent {
    Output(String),
    Exit(i32),
}

impl PtySession {
    pub fn spawn() -> Result<Self, BridgeError> {
        let system = native_pty_system();
        let pair = system
            .openpty(default_size())
            .map_err(|err| BridgeError::protocol(ErrorCode::SpawnFailed, err.to_string()))?;

        let shell = resolve_shell()?;
        let mut command = CommandBuilder::new(&shell);
        command.env("TERM", "xterm-256color");
        command.env("TERMUX_TERMINAL_OBSIDIAN", "1");
        apply_shell_args(&mut command, &shell);

        let child = pair
            .slave
            .spawn_command(command)
            .map_err(|err| BridgeError::protocol(ErrorCode::SpawnFailed, err.to_string()))?;

        drop(pair.slave);

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|err| BridgeError::protocol(ErrorCode::InternalError, err.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|err| BridgeError::protocol(ErrorCode::InternalError, err.to_string()))?;

        let child = Arc::new(Mutex::new(child));
        let writer = Arc::new(Mutex::new(writer));
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        spawn_reader_task(reader, events_tx.clone());
        spawn_wait_task(Arc::clone(&child), events_tx);

        Ok(Self {
            master: pair.master,
            writer,
            child,
            events_rx,
        })
    }

    pub fn write_input(&self, data: String) -> Result<(), BridgeError> {
        let mut writer = self.writer.lock().map_err(|_| {
            BridgeError::protocol(ErrorCode::InternalError, "PTY writer lock was poisoned")
        })?;
        writer.write_all(data.as_bytes()).map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to write to PTY: {err}"),
            )
        })?;
        writer.flush().map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to flush PTY input: {err}"),
            )
        })
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<(), BridgeError> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|err| {
                BridgeError::protocol(
                    ErrorCode::InternalError,
                    format!("Failed to resize PTY: {err}"),
                )
            })
    }

    pub async fn next_event(&mut self) -> Option<PtyEvent> {
        self.events_rx.recv().await
    }

    pub fn close(self) -> Result<(), BridgeError> {
        let mut child = self.child.lock().map_err(|_| {
            BridgeError::protocol(ErrorCode::InternalError, "PTY child lock was poisoned")
        })?;
        child.kill().map_err(|err| {
            BridgeError::protocol(
                ErrorCode::InternalError,
                format!("Failed to terminate shell: {err}"),
            )
        })
    }
}

fn spawn_reader_task(mut reader: Box<dyn Read + Send>, events_tx: mpsc::UnboundedSender<PtyEvent>) {
    tokio::task::spawn_blocking(move || {
        let mut buf = [0_u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(size) => {
                    let output = String::from_utf8_lossy(&buf[..size]).into_owned();
                    if events_tx.send(PtyEvent::Output(output)).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
}

fn spawn_wait_task(
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    events_tx: mpsc::UnboundedSender<PtyEvent>,
) {
    tokio::task::spawn_blocking(move || {
        loop {
            let exit_code = match child.lock() {
                Ok(mut child) => child
                    .try_wait()
                    .ok()
                    .flatten()
                    .map(|status| status.exit_code() as i32),
                Err(_) => Some(-1),
            };

            if let Some(exit_code) = exit_code {
                let _ = events_tx.send(PtyEvent::Exit(exit_code));
                break;
            }

            thread::sleep(Duration::from_millis(50));
        }
    });
}

fn default_size() -> PtySize {
    PtySize {
        rows: DEFAULT_ROWS,
        cols: DEFAULT_COLS,
        pixel_width: 0,
        pixel_height: 0,
    }
}

fn apply_shell_args(command: &mut CommandBuilder, shell: &str) {
    if Path::new(shell)
        .file_name()
        .is_some_and(|name| name == "bash")
    {
        if let Some(home) = env::var_os("HOME") {
            let rcfile = Path::new(&home).join(".termux-terminal.bashrc");
            if rcfile.is_file() {
                command.arg("--rcfile");
                command.arg(rcfile);
            }
        }
    }
    command.arg("-i");
}

fn resolve_shell() -> Result<String, BridgeError> {
    select_shell_from_path(env::var_os("PATH"), default_shell(), "sh").ok_or_else(|| {
        BridgeError::protocol(
            ErrorCode::SpawnFailed,
            "No supported shell found in PATH (tried bash then sh)",
        )
    })
}

fn select_shell_from_path(
    path_value: Option<std::ffi::OsString>,
    preferred: &str,
    fallback: &str,
) -> Option<String> {
    let path_value = path_value?;

    find_executable_in_path(&path_value, preferred)
        .or_else(|| find_executable_in_path(&path_value, fallback))
}

fn find_executable_in_path(path_value: &OsStr, command: &str) -> Option<String> {
    env::split_paths(path_value).find_map(|dir| {
        let candidate = dir.join(command);
        if is_executable_file(&candidate) {
            Some(candidate.to_string_lossy().into_owned())
        } else {
            None
        }
    })
}

fn is_executable_file(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn shell_selection_prefers_bash_over_sh() {
        let path = env::join_paths([PathBuf::from("/bin"), PathBuf::from("/usr/bin")]).unwrap();

        let shell = select_shell_from_path(Some(path), "bash", "sh");
        assert!(
            shell
                .as_deref()
                .is_some_and(|value| value.ends_with("/bash"))
        );
    }

    #[test]
    fn shell_selection_falls_back_to_sh() {
        let path = env::join_paths([PathBuf::from("/bin"), PathBuf::from("/usr/bin")]).unwrap();

        let shell = select_shell_from_path(Some(path), "definitely-not-bash", "sh");
        assert!(shell.as_deref().is_some_and(|value| value.ends_with("/sh")));
    }

    #[test]
    fn shell_selection_returns_none_when_nothing_matches() {
        let path = env::join_paths([PathBuf::from("/definitely/missing")]).unwrap();

        let shell = select_shell_from_path(Some(path), "bash", "sh");
        assert!(shell.is_none());
    }
}
