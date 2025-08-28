use std::io::Write;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn ask_confirmation() -> bool {
    enable_raw_mode().unwrap();

    // Get tty
    let mut tty = match std::fs::OpenOptions::new().write(true).open("/dev/tty") {
        Ok(tty) => tty,
        Err(_) => return false,
    };

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
    tty.flush().unwrap();

    disable_raw_mode().unwrap();

    confirm
}
