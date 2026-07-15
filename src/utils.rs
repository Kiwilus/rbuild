use std::fs;

use std::os::unix::fs::PermissionsExt;

pub fn find_executable(directory:&str) -> Option<String>{

    let entries =
        fs::read_dir(directory)
            .ok()?;

    for entry in entries.flatten(){
        let path =
            entry.path();

        if path.is_dir(){
            if let Some(found)=
                find_executable(
                    path.to_str()?
                )
            {
                return Some(found);
            }
            continue;
        }

        if let Ok(metadata)=
            fs::metadata(&path)
        {
            let executable =
                metadata
                    .permissions()
                    .mode()
                    &
                    0o111
                    !=0;

            if executable{
                return Some(
                    path
                    .to_str()?
                    .to_string()
                );

            }
        }

    }

    None

}