use std::env;
use lrs::{Config,CLI};

fn main() {
    let config: Config = Config::new(env::args().collect());
    let cli: CLI = CLI::from_config(config.clone());

    cli.run();
}
