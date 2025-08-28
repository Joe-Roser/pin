use std::{fs::File, io::Write};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Tty {
    tty: File,
}

impl Tty {
    pub fn new() -> Result<Tty, std::io::Error> {
        enable_raw_mode()?;

        // Get tty
        let tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        Ok(Tty { tty })
    }

    pub fn write(&mut self, msg: String) -> std::io::Result<()> {
        write!(self.tty, "{}", msg.replace("\n", "\n\r"))
    }

    pub fn ask_confirmation(&mut self) -> bool {
        let mut tty = &self.tty;

        let mut confirm = false;

        loop {
            // Handle output
            write!(tty, "\r\x1B[2K").unwrap();
            write!(tty, "Are you sure you want to do this? (y/n)").unwrap();
            tty.flush().unwrap();

            // Handle inputs
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        confirm = true;
                        break;
                    }
                    KeyCode::Esc
                    | KeyCode::Char('n')
                    | KeyCode::Char('N')
                    | KeyCode::Char('q')
                    | KeyCode::Char('Q') => break,
                    _ => {}
                }
            }
        }
        write!(tty, "\n\r").unwrap();

        confirm
    }
}

impl Drop for Tty {
    fn drop(&mut self) {
        self.tty.flush().unwrap();

        disable_raw_mode().unwrap();
    }
}
