use lrs::CLI;
use std::{env, process};

fn main() {
    match CLI::from_args(env::args()) {
        Ok(cli) => cli.run(),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
