
use std::process;
use std::env;
use regex::Regex;

use touch::{ touch_file, show_help, show_version, update_atime, update_mtime, touch_reference };

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    
    let file_name = config.file_name.unwrap();

    if config.show_help {
        show_help();
        return;
    }

    if config.show_version {
        show_version();
        return;
    }

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
                file_name = Some(arg);
            }
        }
       
        if file_name.is_none() {
            return Err("Indique um nome para o ficheiro");
        }

        let regex = Regex::new("[<>:\"/\\\\|?*]").unwrap();

        if regex.is_match(file_name.as_ref().unwrap()) {
            return Err("Caracteres inválidos no nome do ficheiro!");
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

}
