mod cli;
mod logging;

use clap::Parser;
use log::{debug, trace};

use crate::cli::{CLIArguments, Commands};

fn main() {
    logging::init_logger();
    trace!("Logger initialized.");

    let cli_args = CLIArguments::parse();
    debug!("CLI agruments parsed successfully: {:?}", cli_args);

    match &cli_args.command {
        Some(Commands::Test { echo_str }) => {
            println!("Echo: {}", echo_str);
        }
        None => {
            println!("None command received.")
        }
    }
}
