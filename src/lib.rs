
use std::fs;
use std::error::Error;

pub fn create_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(file_name)?;
    Ok(())
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

