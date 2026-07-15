use std::path::Path;

use crate::builder::BuildSystem;

pub fn detect_project() -> Option<BuildSystem> {

    if Path::new("project/Cargo.toml").exists() {
        println!("[+] Rust project detected");
        return Some(BuildSystem::Cargo);
    }

    if Path::new("project/CMakeLists.txt").exists() {
        println!("[+] CMake project detected");
        return Some(BuildSystem::CMake);
    }

    if Path::new("project/Makefile").exists() {
        println!("[+] Make project detected");
        return Some(BuildSystem::Make);
    }

    None
}