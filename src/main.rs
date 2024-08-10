use std::env::{self};

use rs_cli_core::{cat, echo};

fn main() {
    let mut args = env::args();
    let _executing_file = args.next();
    let command = args.next().expect("A command must be provided");

    match &command[..] {
        "cat" => cat(args),
        "echo" => echo(args),
        _ => panic!("Not implemented"),
    };
}
