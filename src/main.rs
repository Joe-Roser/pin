mod cmd;
mod store;

use cmd::*;

use std::env::{Args, args};

fn parse_args(args: Args) -> impl Cmd {
    Help {}
}

fn main() {
    let mut cmd = parse_args(args());
    cmd.execute();
}

//
