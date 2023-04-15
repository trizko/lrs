use lrs::CLI;
use std::env;

fn main() {
    match CLI::from_args(env::args()) {
        Ok(cli) => cli.run(),
        Err(e) => eprintln!("{:?}", e.to_string()),
    }
}
