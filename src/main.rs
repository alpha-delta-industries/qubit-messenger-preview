mod cli;
mod gui;
mod logging;

use clap::Parser;
use log::{debug, error, trace};

use crate::{
    cli::{CLIArguments, Commands},
    gui::App,
};

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
            trace!("Default command received. Running GUI...");
            match App::run() {
                Err(e) => {
                    error!("GUI execution finished with error: {}", e);
                }
                Ok(_) => {
                    trace!("GUI execution finished.");
                }
            }
        }
    }
}
