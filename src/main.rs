mod cli;
mod builder;
mod detector;
mod installer;
mod registry;
mod workspace;
mod utils;

use cli::{Cli, Commands};
use clap::Parser;
use detector::detect_project;
use builder::build_project;
use installer::install;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { url } => {
            workspace::cleanup();

            workspace::clone_repo(&url);

            match detect_project() {
                Some(system) => {
                    build_project(system);

                    install(&url);
                }
                None => {
                    println!("[x] Unknown project type");
                }
            }
            workspace::cleanup();
        }
    }
}