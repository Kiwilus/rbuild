use serde::{Serialize,Deserialize};
use std::fs;

#[derive(Serialize,Deserialize)]
pub struct InstalledPackage {
    pub name:String,
    pub url:String,
    pub install_path:String,

}

pub fn register(
    name:&str,
    url:&str,
    install_path:&str
){

    let home =
        std::env::var("HOME")
            .expect("HOME missing");

    let directory =
        format!(
            "{}/.rbuild",
            home
        );
    fs::create_dir_all(&directory)
        .unwrap();

    let database =
        format!(
            "{}/installed.json",
            directory
        );

    let mut packages:
        Vec<InstalledPackage> =
        fs::read_to_string(&database)
            .ok()
            .and_then(
                |data|
                serde_json::from_str(&data)
                    .ok()
            )
            .unwrap_or_default();

    packages.retain(
        |p|
        p.name != name
    );

    packages.push(
        InstalledPackage {

            name:name.to_string(),

            url:url.to_string(),

            install_path:
                install_path.to_string(),
        }
    );

    let json =
        serde_json::to_string_pretty(
            &packages
        )
        .unwrap();

    fs::write(
        database,
        json
    )
    .unwrap();

    println!(
        "[✓] Registered {}",
        name
    );

}