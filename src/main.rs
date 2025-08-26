mod cmd;
mod store;

use cmd::*;

use std::env::{self, Args, args};
use std::path::PathBuf;

fn parse_path(path: &String) -> Result<PathBuf, &str> {
    if path == &String::from("~") {
        if let Ok(home) = env::var("HOME") {
            let home = PathBuf::from(home);
            Ok(home)
        } else {
            Err("Error: unable to find home directory")
        }
    } else if path.starts_with("~/") && path.len() > 2 {
        if let Ok(home) = env::var("HOME") {
            let mut home = PathBuf::from(home);
            home.push((path.as_str()[2..]).to_string());
            Ok(home)
        } else {
            Err("Error: unable to find home directory")
        }
    } else {
        std::fs::canonicalize(path)
            .map_err(|_| "Error: unable to canonicalise path. Check path exists")
    }
}

fn err_parse_msg(missing: &str, cmd: &str) -> Box<ParseErr> {
    Box::new(ParseErr {
        msg: format!("Error: missing {missing} from command. Please use \"{cmd}\"."),
    })
}

fn parse_args(mut args: Args) -> Box<dyn Cmd> {
    let _ = args.next();

    match args.next().unwrap_or_default().as_str() {
        "--add" | "-a" => {
            // Get the alias or return ParseErr
            let Some(alias) = args.next() else {
                return err_parse_msg("alias", "pin --add [alias] [path]");
            };

            // Get the path or return ParseErr
            let Some(path) = args.next() else {
                return err_parse_msg("alias", "pin --add [alias] [path]");
            };

            // Return an add command
            Box::new(Add { alias, path })
        }
        "--delete" | "-d" => {
            // Get the alias or return ParseErr
            let Some(alias) = args.next() else {
                return err_parse_msg("alias", "pin --delete [alias]");
            };

            //
            Box::new(Delete { alias })
        }
        "--help" | "-h" => Box::new(Help { cmd: args.next() }),
        "--list" | "-l" => Box::new(List {}),
        "--update" | "-u" => {
            // Get the alias or return ParseErr
            let Some(alias) = args.next() else {
                return err_parse_msg("alias", "pin --update [alias]");
            };

            Box::new(Update { alias })
        }
        catch if catch.starts_with("-") => Box::new(ParseErr {
            msg: format!(
                "No function named {}, type --help to list all commands.",
                catch
            ),
        }),
        alias => Box::new(Pin {
            alias: alias.to_string(),
        }),
    }
}

// Entry point for the program
fn main() {
    let cmd = parse_args(args());
    let (msg, code) = cmd.execute();
    println!("{}", msg);
    std::process::exit(code);
}

//
