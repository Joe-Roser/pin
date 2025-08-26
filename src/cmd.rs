// The commands that can be passed into the program

use crate::store::Store;

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
    const USAGE: &str = "pin --add [alias] [path]";
    const DESC: &str = "Add the supplied alias to the supplied path.";
}

impl Cmd for Add {
    fn execute(self: Box<Self>) -> (String, i32) {
        // Test path
        let path = match crate::parse_path(&self.path) {
            Ok(path) => path.into_os_string().into_string().unwrap(),
            Err(e) => return (e.to_string(), 1),
        };

        let mut store = Store::init();
        // TODO:
        let _ = store.add(self.alias, path);
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
    const USAGE: &str = "pin --delete [alias]";
    const DESC: &str = "Delete an alias from it's store.";
}

impl Cmd for Delete {
    fn execute(self: Box<Self>) -> (String, i32) {
        // Ask for confirmation

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
    const USAGE: &str = "pin --help [cmd(optional)]";
    const DESC: &str = "List all commands for pin.";
}

impl Cmd for Help {
    fn execute(self: Box<Self>) -> (String, i32) {
        let help = match self.cmd.as_deref() {
            Some("add") => format!("{}\n{}\n{}", Add::NAME, Add::USAGE, Add::DESC),
            Some("delete") => format!("{}\n{}\n{}", Delete::NAME, Delete::USAGE, Delete::DESC),
            Some("list") => format!("{}\n{}\n{}", List::NAME, List::USAGE, List::DESC),
            Some("update") => format!("{}\n{}\n{}", Update::NAME, Update::USAGE, Update::DESC),
            Some("pin") => format!("{}\n{}\n{}", Pin::NAME, Pin::USAGE, Pin::DESC),
            Some(other) => format!("Unknown command: {other}"),
            None => {
                // Show all commands
                vec![
                    format!("{} - {}\n", Pin::USAGE, Pin::DESC),
                    format!("{} - {}\n", Add::USAGE, Add::DESC),
                    format!("{} - {}\n", Delete::USAGE, Delete::DESC),
                    format!("{} - {}\n", List::USAGE, List::DESC),
                    format!("{} - {}\n", Update::USAGE, Update::DESC),
                    format!("{} - {}\n", Help::USAGE, Help::DESC),
                ]
                .join("")
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
    const USAGE: &str = "pin --list";
    const DESC: &str = "List all alias-path pairs";
}

impl Cmd for List {
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
    const USAGE: &str = "pin --update";
    const DESC: &str = "Update an alias-path pair";
}

impl Cmd for Update {
    fn execute(self: Box<Self>) -> (String, i32) {
        todo!()
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
