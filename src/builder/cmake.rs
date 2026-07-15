use std::process::Command;

pub fn build(){
    println!("[+] Configuring cmake");

    let configure =
        Command::new("cmake")
            .arg("-S")
            .arg("project")
            .arg("-B")
            .arg("project/build")
            .status();

    match configure {
        Ok(s) if s.success()=>{
            println!("[✓] Configure successful");
        }
        _=>{
            println!("[x] CMake configure failed");
            std::process::exit(1);

        }
    }
    println!("[+] Building cmake");

    let build =
        Command::new("cmake")
            .arg("--build")
            .arg("project/build")
            .status();

    match build {
        Ok(s) if s.success()=>{
            println!("[✓] Build successful");
        }
        _=>{
            println!("[x] CMake build failed");
            std::process::exit(1);

        }
    }

}