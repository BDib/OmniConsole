use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::thread;

/// Represents an active Pseudo-Terminal (PTY) session, wrapping a platform-native Master/Slave pair
/// and managing input/output operations to the shell subprocess.
pub struct Pty {
    writer: Box<dyn Write + Send>,
    reader: Option<Box<dyn Read + Send>>,
    #[allow(dead_code)]
    child: Box<dyn portable_pty::Child + Send>,
    #[allow(dead_code)]
    pair: portable_pty::PtyPair,
    size: PtySize,
}

/// Events emitted by the asynchronous PTY reader thread.
pub enum PtyEvent {
    /// Received standard output data from the shell subprocess.
    Output(String),
    /// Subprocess has exited or closed the terminal interface.
    Exited,
}

impl Pty {
    pub fn new(command: &str, args: &[String], cols: u16, rows: u16) -> Result<Self, String> {
        let pty_system = NativePtySystem::default();
        
        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        
        let pair = pty_system.openpty(size).map_err(|e| e.to_string())?;
        
        let mut cmd = CommandBuilder::new(command);
        for arg in args {
            cmd.arg(arg);
        }
        
        let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
        
        let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
        let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
        
        Ok(Pty {
            writer,
            reader: Some(reader),
            child,
            pair,
            size,
        })
    }

    pub fn spawn_reader(mut self) -> (tokio::sync::mpsc::Receiver<PtyEvent>, Self) {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        let reader = self.reader.take().unwrap();
        
        thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => {
                        let _ = tx.blocking_send(PtyEvent::Exited);
                        break;
                    }
                    Ok(n) => {
                        let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                        if tx.blocking_send(PtyEvent::Output(text)).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        let _ = tx.blocking_send(PtyEvent::Exited);
                        break;
                    }
                }
            }
        });
        
        (rx, self)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), String> {
        self.writer.write_all(data).map_err(|e| e.to_string())?;
        self.writer.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn write_str(&mut self, data: &str) -> Result<(), String> {
        self.write(data.as_bytes())
    }
}
