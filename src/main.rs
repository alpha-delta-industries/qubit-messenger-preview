mod cli;

use clap::Parser;
use log::trace;

use crate::cli::{CLIArguments, Commands};

fn main() {
    pretty_env_logger::init();
    trace!("Logger initialized.");

    let cli_args = CLIArguments::parse();
    trace!("CLI agruments parsed successfully: {:?}", cli_args);

    match &cli_args.command {
        Some(Commands::Test { echo_str }) => {
            println!("Echo: {}", echo_str);
        }
        None => {
            println!("None command received.")
        }
    }
}
