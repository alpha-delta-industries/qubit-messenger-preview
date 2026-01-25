use clap::Parser;
use log::{debug, info};

use crate::{
    Cli::{CLIArguments, Commands},
    peer::start_peer,
    webui::start_webui,
};

mod Cli;

mod peer;
mod webui;

fn main() {
    pretty_env_logger::init();
    info!("Qubit {}", env!("CARGO_PKG_VERSION"));

    let cli_args = CLIArguments::parse();
    debug!("CLI agruments parsed successfully: {:?}", cli_args);

    match &cli_args.command {
        Some(Commands::Test { echo_str }) => {
            println!("Echo: {}", echo_str);
        }
        Some(Commands::StartPeer {}) => {
            start_peer();
        }
        Some(Commands::StartWebUI {}) => {
            start_webui();
        }
        None => {
            println!("None command received.")
        }
    }
}
