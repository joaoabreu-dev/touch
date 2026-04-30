
use std::fs::{self, FileTimes};
use std::error::Error;
use std::time::SystemTime;

#[deprecated(since = "0.4.0", note = "Use touch_file as it has the expected behaviour")]
pub fn create_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    fs::File::create(file_name)?;
    Ok(())
}  

fn ensure_file_exists(file_name: &str, create_if_missing: bool) -> Result<(), Box<dyn Error>> {
    if fs::exists(file_name)? {
        return Ok(());
    }

    if create_if_missing {
        fs::File::create(file_name)?;
        Ok(())
    } else {
        Err(format!("File {} doesn't exist.", file_name).into())
    }
}

pub fn touch_file(file_name: &str, create_file: bool) -> Result<(), Box<dyn Error>> {
    ensure_file_exists(file_name, create_file)?;
        
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

pub fn touch_reference(dest: &str, refer: &str, create: bool) -> Result<(), Box<dyn Error>> {
    ensure_file_exists(dest, create)?;

    ensure_file_exists(refer, false)?; 

    let file_handle = fs::OpenOptions::new()
                        .write(true)
                        .open(dest)?;

    let metadata = fs::metadata(refer)?;

    let times = FileTimes::new() 
        .set_accessed(metadata.accessed().unwrap())
        .set_modified(metadata.modified().unwrap());

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
    println!("
Usage: touch [OPTION] FILE
Update the access and modification times of the file.

Available options:
    -a                  change only the access time of the file
    -c                  do not create any files
    -m                  change only the modification time of the file
    -r REFERENCE_FILE   change access and modification time of a file based on another
    
    --help              display this help and exit
    --version           display version information and exit

Examples:
    touch file.txt
    touch -c file.txt
    touch -r source.txt dest.txt
    touch -m file.txt

Full documentation and source code on https://github.com/joaoabreu-dev/touch
For any questions or suggestions regarding the program, please contact me on jpfernandes.a139@gmail.com.

    ");
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
        let file_name = "tests/touch_test.txt";

        let result_with_create = touch_file(file_name, true);
        let result_without_create = touch_file(file_name, false);

        assert!(result_with_create.is_ok(), "Esperava successo, mas deu erro: {:?}", result_with_create);
        assert!(result_without_create.is_ok(), "Esperava sucesso, mas deu erro: {:?}", result_without_create);
    }
    
    #[test]
    fn test_update_atime() {
        let file_name = "tests/update_atime_test.txt";
        
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
        let file_name = "tests/update_mtime_test.txt";

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

    #[test]
    fn test_touch_reference() {
        let file_name = "tests/touch_reference_test.txt";
        let reference = "Cargo.toml";
        
        let result_w_create = touch_reference(file_name, reference, true);
        let result_n_create = touch_reference(file_name, reference, false);

        assert!(result_w_create.is_ok(), "Esperava sucesso, mas deu erro: {:?}", result_w_create);
        assert!(result_n_create.is_ok(), "Esperava sucesso, mas deu erro: {:?}", result_n_create);

        let metadata = fs::metadata(reference).unwrap();
        let ref_times = vec![metadata.accessed().unwrap(), metadata.modified().unwrap()];

        let metadata = fs::metadata(file_name).unwrap();
        let dest_times = vec![metadata.accessed().unwrap(), metadata.modified().unwrap()];

        assert_eq!(ref_times, dest_times);
    }

    #[test]
    fn test_file_exists() {
        let file_name = "tests/test_file_exists.txt";
    
        let ensure = ensure_file_exists(file_name, true);

        assert!(ensure.is_ok());
    }
}

