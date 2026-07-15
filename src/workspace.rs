use std::fs;
use std::process::Command;



pub fn clone_repo(url:&str){

    println!("[+] Cloning repository");

    let result =
        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg("project")
            .status();

    match result {
        Ok(status) if status.success()=>{
            println!("[✓] Clone finished");
        }

        Ok(_)=>{
            println!("[x] Git clone failed");
            std::process::exit(1);
        }
        Err(e)=>{
            println!("[x] Git error: {}",e);
            std::process::exit(1);
        }

    }

}

pub fn cleanup(){

    if fs::metadata("project").is_ok(){
        println!("[+] Cleaning workspace");

        let _ =
            fs::remove_dir_all("project");
    }

}