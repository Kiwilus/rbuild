use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;
use std::fs;

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
        url: String,
    },
}

enum BuildSystem {
    Cargo,
    CMake,

}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { url } => {
            clone_repo(&url);
            let build_system = detect_project();
            match build_system {
                Some(BuildSystem::Cargo) => {
                    build_project();
                    install_binary();
                }
                Some(BuildSystem::CMake) => println!("CMake build not implemented yet"),
                None => {
                    println!("Unknown project type");
                    return;
                }
            }
        }
    }
}

fn get_project_name() -> Option<String> {
    let content = fs::read_to_string("project/Cargo.toml").ok()?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            return line.split('=').nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn clone_repo(url: &str) {

    println!("[+] Cloning repository");

    let result = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg("project")
        .status();

    match result {

        Ok(status) => {

            if status.success() {
                println!("[✓] Clone finished");
            }

        }
        Err(error) => {
            println!("Error: {}", error);
        }

    }
}

fn detect_project() -> Option<BuildSystem> {

    if Path::new("project/Cargo.toml").exists() {
        println!("[+] Rust project detected");
        return Some(BuildSystem::Cargo);
    }

    if Path::new("project/CMakeLists.txt").exists() {
        println!("[+] CMake project detected");
        return Some(BuildSystem::CMake);
    }


    None
}

fn build_project(){

    println!("[+] Building");


    match Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir("project")
        .status()
    {
        Ok(status) => {
            if status.success() {
                println!("[✓] Build successful");
            } else {
                println!("[x] Build failed");
                std::process::exit(1);
            }
        }

        Err(e) => {
            println!("Build error: {}", e);
        }
    }


}

fn install_binary() {
    let name = match get_project_name() {
        Some(n) => n,
        None => {
            println!("[x] Could not determine project name");
            std::process::exit(1);
        }
    };

    let home = std::env::var("HOME").expect("HOME not set");
    let install_dir = format!("{}/.local/bin", home);
    fs::create_dir_all(&install_dir).unwrap();

    let src = format!("project/target/release/{}", name);
    let dest = format!("{}/{}", install_dir, name);

    match fs::copy(&src, &dest) {
        Ok(_) => println!("[✓] Installed to {}", dest),
        Err(e) => {
            println!("[x] Install failed: {}", e);
            std::process::exit(1);
        }
    }
}