
use std::fs::{self, FileTimes};
use std::error::Error;
use std::time::SystemTime;

#[deprecated(since = "0.4.0", note = "Use touch_file as it has the expected behaviour")]
pub fn create_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    fs::File::create(file_name)?;
    Ok(())
}  

pub fn touch_file(file_name: &str, create_file: bool) -> Result<(), Box<dyn Error>> {
    
    if !fs::exists(file_name).unwrap() {
        if create_file {
            fs::File::create(file_name)?;
            return Ok(());
        } else {
            return Err("Ficheiro não existe!".into());
        }
    }
        
    let file_handle = fs::OpenOptions::new()
                     .write(true)
                     .open(file_name)?;

    let now = SystemTime::now();

    let times = FileTimes::new()
                .set_accessed(now)
                .set_modified(now);

    file_handle.set_times(times)?;

    Ok(())
}

pub fn update_atime(file_name: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(file_name)?;
    let file_handle = fs::OpenOptions::new()
                          .write(true)
                          .open(file_name)?;

    let mtime = metadata.modified()?;

    let now = SystemTime::now();

    let times = FileTimes::new()
        .set_accessed(now)
        .set_modified(mtime);

    file_handle.set_times(times)?;

    Ok(())
}

pub fn update_mtime(file_name: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(file_name)?;
    let file_handle = fs::OpenOptions::new()
                          .write(true)
                          .open(file_name)?;

    let atime = metadata.accessed()?;

    let now = SystemTime::now();

    let times = FileTimes::new()
                .set_accessed(atime)
                .set_modified(now);

    file_handle.set_times(times)?;

    Ok(())
}

pub fn show_help() {
    unimplemented!()
}

pub fn show_version() {
    println!("
Rusty Touch
Rust implementation of the touch command for Windows.
As this is my adaptation of an already existing and very widespread tool, you are free to change and redistribute it.
Version: {}


Author: João Abreu
Feel free to contact me: jpfernandes.a139@gmail.com.
Thank you for using!
        ", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_file() {
        let file_name = "touch_test.txt";

        //let result_with_create = touch_file(file_name, true);
        let result_without_create = touch_file(file_name, false);

        //assert!(result_with_create.is_ok(), "Esperava successo, mas deu erro: {:?}", result_with_create);
        assert!(result_without_create.is_ok(), "Esperava sucesso, mas deu erro: {:?}", result_without_create);
    }

    #[test]
    fn test_update_atime() {
        let file_name = "update_atime_test.txt";
        
        if !fs::exists(file_name).unwrap() {
            create_file(file_name);
        }

        let old_mtime = fs::metadata(file_name).unwrap().modified().unwrap();

        let result = update_atime(file_name);       
        let now = SystemTime::now();

        assert!(result.is_ok(), "Esperava sucesso, mas deu erro: {:?}", result);
        let metadata = fs::metadata(file_name).unwrap();
        let atime = metadata.accessed().unwrap();

        let difference = now.duration_since(atime).unwrap();

        assert_eq!(old_mtime, metadata.modified().unwrap());
        assert!(difference.as_secs() < 2, "atime está demasiado desatualizado {:?}", difference);
    }

    #[test]
    fn test_update_mtime() {
        let file_name = "filee.txt";

        if !fs::exists(file_name).unwrap() {
            create_file(file_name);
        }

        let old_atime = fs::metadata(file_name).unwrap().accessed().unwrap();


        let result = update_mtime(file_name);
        let now = SystemTime::now();

        assert!(result.is_ok(), "Esparava sucesso, mas deu erro: {:?}", result);

        let metadata = fs::metadata(file_name).unwrap();
        let mtime = metadata.modified().unwrap();

        let difference = now.duration_since(mtime).unwrap();

        assert_eq!(old_atime, metadata.accessed().unwrap());
        assert!(difference.as_secs() < 2, "mtime está demasiado desatualizado {:?}", difference);
    }
}

