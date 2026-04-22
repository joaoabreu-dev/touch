
use std::fs;
use std::error::Error;

pub fn create_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    fs::File::create(file_name)?;
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
    fn test_create_file() {
        let file_name = "file.txt";
        
        let result = create_file(file_name);

        assert!(result.is_ok(), "Esperava successo, mas deu erro: {:?}", result);
    }
}

