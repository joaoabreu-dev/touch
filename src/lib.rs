
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
    unimplemented!()
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

