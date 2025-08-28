// The commands that can be passed into the program

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use prettytable::{Table, row};

use crate::{parse_path, store::Store};

// Trait for structs that can be executed. Each command should implement execute. This is vv
// similar to how I would use message enums normally, but with named parameters vv easily and
// seperate implementations of execute without having to pass functions as parameters. Win.
//

pub trait Cmd {
    fn execute(self: Box<Self>) -> (String, i32);
}

// Implementors of Cmd

// pin <alias>
//
// Used to jump to a string by the alias
pub struct Pin {
    pub alias: String,
}

impl Pin {
    const NAME: &str = "pin";
    const USAGE: &str = "pin [alias]";
    const DESC: &str = "Go to the path aliased";
}

impl Cmd for Pin {
    // Return path or error
    fn execute(self: Box<Self>) -> (String, i32) {
        let store = Store::init();
        match store.get(&self.alias) {
            Some(path) => (path, 3),
            None => (
                String::from("Error: Alias not found in store. Type \"pin --help\" for help"),
                1,
            ),
        }
    }
}

// pin --add <alias> <path>
//
// Used to add a new alias to the store
pub struct Add {
    pub alias: String,
    pub path: String,
}

impl Add {
    const NAME: &str = "add";
    const SHORT: &str = "-a";
    const USAGE: &str = "pin --add [alias] [path]";
    const DESC: &str = "Add the supplied alias to the supplied path.";
}

impl Cmd for Add {
    // Add an alias-path pair
    fn execute(self: Box<Self>) -> (String, i32) {
        // Test path
        let path = match crate::parse_path(&self.path) {
            Ok(path) => path.into_os_string().into_string().unwrap(),
            Err(e) => return (e.to_string(), 1),
        };

        let mut store = Store::init();

        // Confirmation on overwriting alias
        // HACK: Defo refactor this
        match store.add(self.alias, path) {
            Err(path) => {
                {
                    let mut tty = match std::fs::OpenOptions::new().write(true).open("/dev/tty") {
                        Ok(file) => file,
                        Err(_) => return (String::from("Error: unable to access /dev/tty"), 1),
                    };
                    use std::io::Write;

                    write!(
                        tty,
                        "This action will overwrite existing alias to {}.\n\r",
                        path
                    )
                    .unwrap();
                }

                match crate::tty::ask_confirmation() {
                    true => {}
                    false => return (String::new(), 0),
                }
            }
            _ => {}
        }

        // Save changes and exit
        store.save();
        (String::default(), 0)
    }
}

// pin --delete <alias>
//
// Used to delete an alias from the store
pub struct Delete {
    pub alias: String,
}

impl Delete {
    const NAME: &str = "delete";
    const SHORT: &str = "-d";
    const USAGE: &str = "pin --delete [alias]";
    const DESC: &str = "Delete an alias from it's store.";
}

impl Cmd for Delete {
    // Delete a path
    fn execute(self: Box<Self>) -> (String, i32) {
        if !crate::tty::ask_confirmation() {
            return (String::new(), 0);
        }
        let mut store = Store::init();
        let ok = store.delete(self.alias).is_ok();
        store.save();
        if ok {
            (String::default(), 0)
        } else {
            (String::from("Error: Alias not found in store"), 1)
        }
    }
}

// pin --help
//
// Used to list all possible commands
pub struct Help {
    pub cmd: Option<String>,
}

impl Help {
    const NAME: &str = "help";
    const SHORT: &str = "-h";
    const USAGE: &str = "pin --help [cmd(optional)]";
    const DESC: &str = "List all commands for pin or get specifics about one command.";
}

impl Cmd for Help {
    // Print out help for all commands
    fn execute(self: Box<Self>) -> (String, i32) {
        let help = match self.cmd.as_deref() {
            Some("add") => format!(
                "{} ({}):\n  Usage: {}\n  Description: {}",
                Add::NAME,
                Add::SHORT,
                Add::USAGE,
                Add::DESC
            ),
            Some("delete") => format!(
                "{} ({}):\n  Usage: {}\n  Description: {}",
                Delete::NAME,
                Delete::SHORT,
                Delete::USAGE,
                Delete::DESC
            ),
            Some("list") => format!(
                "{} ({}):\n  Usage: {}\n  Description: {}",
                List::NAME,
                List::SHORT,
                List::USAGE,
                List::DESC
            ),
            Some("update") => format!(
                "{} ({}):\n  Usage: {}\n  Description: {}",
                Update::NAME,
                Update::SHORT,
                Update::USAGE,
                Update::DESC
            ),
            Some("pin") => format!(
                "{}:\n  Usage: {}\n  Description: {}",
                Pin::NAME,
                Pin::USAGE,
                Pin::DESC
            ),
            Some("help") => format!(
                "{} ({}):\n  Usage: {}\n  Description: {}",
                Help::NAME,
                Help::SHORT,
                Help::USAGE,
                Help::DESC
            ),
            Some(other) => format!("Unknown command: {}", other),
            // Show summary of all commands
            None => {
                let mut table = Table::new();

                table.add_row(row!["Command", "Description", "Usage"]);
                table.add_row(row![Pin::NAME, Pin::DESC, Pin::USAGE]);
                table.add_row(row![Add::NAME, Add::DESC, Add::USAGE]);
                table.add_row(row![Delete::NAME, Delete::DESC, Delete::USAGE]);
                table.add_row(row![Help::NAME, Help::DESC, Help::USAGE]);
                table.add_row(row![List::NAME, List::DESC, List::USAGE]);
                table.add_row(row![Update::NAME, Update::DESC, Update::USAGE]);

                table.to_string()
            }
        };

        (help, 2)
    }
}

// pin --list
//
// Used to list all alias path pairs
pub struct List {}

impl List {
    const NAME: &str = "list";
    const SHORT: &str = "-l";
    const USAGE: &str = "pin --list";
    const DESC: &str = "List all alias-path pairs";
}

impl Cmd for List {
    // List all current aliases
    fn execute(self: Box<Self>) -> (String, i32) {
        let store = Store::init();
        (store.list_all(), 2)
    }
}

// pin --update
//
// Used to update an alias path pair
pub struct Update {
    pub alias: String,
}

impl Update {
    const NAME: &str = "update";
    const SHORT: &str = "-u";
    const USAGE: &str = "pin --update [alias]";
    const DESC: &str = "Update an alias-path pair";
}

impl Cmd for Update {
    // Update a pair, with some tui
    fn execute(self: Box<Self>) -> (String, i32) {
        // Check store to make sure the alias is valid
        let mut store = Store::init();

        let path = match store.get(&self.alias) {
            Some(path) => path,
            None => {
                return (
                    String::from(
                        "Error: alias not in store. You can't update an alias that doesn't exist.",
                    ),
                    1,
                );
            }
        };

        // Try get access to writing directly to terminal
        use std::io::Write;
        let mut tty = match std::fs::OpenOptions::new().write(true).open("/dev/tty") {
            Ok(tty) => tty,
            Err(_) => return (String::from("Error: unable to access /dev/tty"), 1),
        };

        // Intercept inputs before they go to the terminal so we can handle them manually
        enable_raw_mode().unwrap();

        let options = ["alias (a)", "path (p)"];
        let mut selected = 0;

        // Loop till choice is made or escape
        loop {
            // Handle output
            write!(tty, "\r\x1B[2K").unwrap();
            for (i, option) in options.iter().enumerate() {
                if i == selected {
                    write!(tty, "\x1b[38:5:41m[{}]\x1b[0m", option).unwrap();
                } else {
                    write!(tty, " {} ", option).unwrap();
                }
            }
            tty.flush().unwrap();

            // Handle input
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Enter => break,
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        selected = 0;
                        break;
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        selected = 1;
                        break;
                    }
                    KeyCode::Left | KeyCode::Char('h') => selected = 0,
                    KeyCode::Right | KeyCode::Char('l') => selected = 1,
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        return (String::from("Canceled"), 2);
                    }
                    _ => {}
                }
            }
        }
        write!(tty, "\r\n").unwrap();

        // Handle choice made
        let option = options[selected];
        let mut input = if selected == 0 {
            self.alias.clone()
        } else {
            path.clone()
        };
        let mut cursor_position = input.len();
        let offset = option.len() + 3;

        loop {
            // Handle output
            write!(tty, "\r\x1B[2K").unwrap();
            write!(tty, "{}: {}", option, input).unwrap();
            write!(tty, "\r\x1B[{}G", offset + cursor_position).unwrap();
            tty.flush().unwrap();

            //Handle Input
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Enter => break,
                    KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        return (String::from("Canceled"), 2);
                    }
                    KeyCode::Backspace => {
                        if cursor_position > 0 {
                            input.remove(cursor_position - 1);
                            cursor_position -= 1;
                        }
                    }
                    KeyCode::Left => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_position < input.len() {
                            cursor_position += 1;
                        }
                    }
                    KeyCode::Char(char) => {
                        input.insert(cursor_position, char);
                        cursor_position += 1;
                    }
                    _ => {}
                }
            }
        }
        write!(tty, "\r\n").unwrap();

        disable_raw_mode().unwrap();

        if selected == 0 {
            store.delete(self.alias).unwrap();
            let _ = store.add(input, path);
            store.save();
        } else {
            let input = match parse_path(&input) {
                Ok(path) => path.into_os_string().into_string().unwrap(),
                Err(msg) => return (msg.to_string(), 1),
            };
            let _ = store.add(self.alias, input);
            store.save();
        }

        (String::new(), 0)
    }
}

// pin _
//
// Catch initial errors
pub struct ParseErr {
    pub msg: String,
}

impl Cmd for ParseErr {
    fn execute(self: Box<Self>) -> (String, i32) {
        (self.msg, 1)
    }
}

//
