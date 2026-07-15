use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="rbuild")]
#[command(version)]
#[command(about="Build software from Git repositories")]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        url: String,
    },
}