use rs_cli_core::echo;
use std::env::{self};

fn main() {
    let mut args = env::args();
    let _executing_file = args.next();
    let command = args.next().expect("A command must be provided");

    if command == "echo" {
        println!("{}", echo::handler(args));
    }
}
