
use std::process;
use std::env;

use touch::{ touch_file, show_help, show_version, update_atime, update_mtime, touch_reference };

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    
    if config.show_help {
        show_help();
        return;
    }

    if config.show_version {
        show_version();
        return;
    }

    let file_name = config.file_name.unwrap();

    if config.use_reference {
        touch_reference(&file_name, &config.reference.unwrap(), config.create_file).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    }

    if !config.update_atime && !config.update_mtime {
        touch_file(&file_name, config.create_file).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    }

    if config.update_atime {
        update_atime(&file_name).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    }

    if config.update_mtime {
        update_mtime(&file_name).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    }
}

#[derive(Debug)]
struct Config {
    file_name: Option<String>,
    show_help: bool,
    show_version: bool,
    update_atime: bool,
    update_mtime: bool,
    create_file: bool,
    use_reference: bool,
    reference: Option<String>
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 
            
        let mut show_help = false;
        let mut show_version = false;
        let mut update_atime = false;
        let mut update_mtime = false;
        let mut create_file = true;
        let mut use_reference = false;
        let mut reference: Option<String> = None;
        let mut file_name: Option<String> = None;
        
        while let Some(arg) = args.next() {
            if arg.starts_with("-") {
                match &arg[..] {
                    "--help" | "-h" => {
                        show_help = true;
                        create_file = false;
                        return Ok(Config { file_name, show_help, show_version, update_atime, update_mtime, create_file, use_reference, reference });
                    },
                    "--version" => {
                        show_version = true;
                        create_file = false;
                        return Ok(Config { file_name, show_help, show_version, update_atime, update_mtime, create_file, use_reference, reference });
                    },
                    "-a" => { 
                        update_atime = true;
                    },
                    "-c" => {
                        create_file = false;
                    },
                    "-m" => {
                        update_mtime = true;
                    },
                    "-r" => {
                        use_reference = true;
                        reference = match args.next() {
                            Some(arg) => {
                               Some(arg) 
                            },
                            None => {
                                return Err("Ficheiro de referência esperado!");
                            }
                        }
                    }
                    _ => return Err("Flag inválida!"), 
                };
            } else {
                Self::is_file_name_valid(&arg).map_err(|e| e)?; 
                file_name = Some(arg);
            }
        }
       
        if file_name.is_none() {
            return Err("Indique um nome para o ficheiro");
        }
        
        Ok(Config {
            file_name,
            show_help,
            show_version,
            update_atime,
            update_mtime,
            create_file,
            use_reference,
            reference
        })

    }

    fn is_file_name_valid(file_name: &str) -> Result<(), &'static str> {
        
        if file_name.len() > 255 {
            return Err("O número máximo de caracteres permitido é 255.");
        }

        let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        if file_name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err("O nome do ficheiro contém caracteres inválidos.");
        }

        let reserved_names = [
            "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        ];

        let base_name = file_name.split('.').next().unwrap_or("").to_uppercase();
        if reserved_names.contains(&base_name.as_str()) {
            return Err("O nome do ficheiro é reservado pelo sistema.");
        }
        
        Ok(())
    }

}
