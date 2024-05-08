mod args_parser;
mod display;
mod filters;
mod sort;
mod walker;

use args_parser::{Args, Config};
use walker::start_walking;

fn main() -> std::io::Result<()> {
    let args = <Args as clap::Parser>::parse();
    let config = Config::new_from_args(args);

    start_walking(config)
}
