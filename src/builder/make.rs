use std::process::Command;


pub fn build(){
    println!("[+] Building (make)");

    let result =
        Command::new("make")
            .current_dir("project")
            .status();

    match result {
        Ok(s) if s.success()=>{
            println!("[✓] Make build successful");
        }

        Ok(_)=>{
            println!("[x] Make failed");
            std::process::exit(1);
        }

        Err(e)=>{
            println!("[x] Make error {}",e);
            std::process::exit(1);
        }
    }

}