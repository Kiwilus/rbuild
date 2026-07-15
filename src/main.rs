use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rbuild")]
#[command(version)]
#[command(about = "Build software from Git repositories")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        repo: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { repo } => {
            println!("Installing {}", repo);

            clone_repo(&repo);

            println!("Done!");
        }
    }
}

fn clone_repo(repo: &str) {
    println!("Cloning repository...");
    println!("Repository: {}", repo);

    // TODO: Actually clone the repository
}