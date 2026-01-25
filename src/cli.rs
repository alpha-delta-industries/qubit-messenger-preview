use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Test {
        #[arg(short, long)]
        echo_str: String,
    },
    StartPeer {},
    StartWebUI {},
}

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Simple, free and open-source P2P-messenger."
)]
pub struct CLIArguments {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}
