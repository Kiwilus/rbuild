use std::process::Command;

use crate::builder::cargo;

use crate::registry::register;

use crate::utils::find_executable;



pub fn install(url:&str){

    let package_name =
        match cargo::get_package_name(){
            Some(n)=>n,
            None=>{
                println!("[x] Package name missing");
                std::process::exit(1);

            }
        };

    let binary =
        cargo::get_bin_name()
            .unwrap_or(package_name.clone());

    let source =
        match cargo::find_binary(&binary){
            Some(path)=>path,
            None=>{
                match find_executable("project"){
                    Some(path)=>path,
                    None=>{
                        println!("[x] No executable found");
                        std::process::exit(1);

                    }
                }
            }
        };

    install_file(
        &source,
        &binary,
        url
    );

}


fn install_file(
    source:&str,
    name:&str,
    url:&str
){

    let destination =
        format!(
            "/usr/local/bin/{}",
            name
        );

    println!(
        "[+] Installing {}",
        destination
    );

    let result =
        Command::new("sudo")
            .arg("cp")
            .arg(source)
            .arg(&destination)
            .status();


    match result {
        Ok(status) if status.success()=>{
            println!(
                "[✓] Installed {}",
                name
            );

            register(name,url,&destination);
        }
        _=>{
            println!("[x] Installation failed");
            std::process::exit(1);

        }

    }

}