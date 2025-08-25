// The commands that can be passed into the program

// Trait for structs that can be executed. Each command should implement execute. This is vv
// similar to how I would use message enums normally, but with named parameters vv easily and
// seperate implementations of execute without having to pass functions as parameters. Win.
//
pub trait Cmd {
    fn execute(&mut self);
}

// Implementors of Cmd

// pin <alias>
//
// Used to jump to a string by the alias
pub struct Pin {
    alias: String,
}

impl Cmd for Pin {
    fn execute(&mut self) {}
}

// pin --add <alias> <path>
//
// Used to add a new alias to the store
pub struct Add {
    alias: String,
    path: String,
}

impl Cmd for Add {
    fn execute(&mut self) {}
}

// pin --delete <alias>
//
// Used to delete an alias from the store
pub struct Delete {
    alias: String,
}

impl Cmd for Delete {
    fn execute(&mut self) {}
}

// pin --help
//
// Used to list all possible commands
pub struct Help {}

impl Cmd for Help {
    fn execute(&mut self) {}
}

// pin --list
//
// Used to list all alias path pairs
pub struct List {}

impl Cmd for List {
    fn execute(&mut self) {}
}

// pin --update
//
// Used to update an alias path pair
pub struct Update {}

impl Cmd for Update {
    fn execute(&mut self) {}
}
