use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build() {
    println!("[+] Building (cargo)");

    let result = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir("project")
        .status();

    match result {
        Ok(status) if status.success() => {
            println!("[✓] Cargo build successful");
        }
        Ok(_) => {
            println!("[x] Cargo build failed");
            std::process::exit(1);
        }
        Err(e) => {
            println!("[x] Cargo error: {}", e);
            std::process::exit(1);

        }
    }

}

pub fn get_package_name() -> Option<String> {
    let content =
        fs::read_to_string("project/Cargo.toml")
            .ok()?;

    let mut inside = false;

    for line in content.lines() {
        let line = line.trim();

        if line == "[package]" {
            inside = true;
            continue;
        }

        if line.starts_with('[') {
            inside = false;
        }

        if inside && line.starts_with("name") {
            return line
                .split('=')
                .nth(1)
                .map(|x|
                    x.trim()
                    .trim_matches('"')
                    .to_string()
                );

        }

    }



    None
}

pub fn get_bin_name() -> Option<String> {
    let content =
        fs::read_to_string("project/Cargo.toml")
            .ok()?;

    let mut inside = false;

    for line in content.lines() {
        let line=line.trim();

        if line=="[[bin]]" {
            inside=true;
            continue;
        }

        if line.starts_with('[') {
            inside=false;
        }

        if inside && line.starts_with("name") {
            return line
                .split('=')
                .nth(1)
                .map(|x|
                    x.trim()
                    .trim_matches('"')
                    .to_string()
                );
        }

    }
    get_package_name()

}

pub fn find_binary(name:&str)->Option<String>{
    let dir="project/target/release";

    let direct=format!("{}/{}",dir,name);

    if Path::new(&direct).exists(){
        return Some(direct);
    }

    let underscore=name.replace("-","_");

    let alternative=
        format!("{}/{}",dir,underscore);

    if Path::new(&alternative).exists(){
        return Some(alternative);
    }

    None
}