
use std::process;
use std::env;
use regex::Regex;

use touch::{ create_file, show_help, show_version };

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

    if config.create_file {
        create_file(config.file_name.unwrap().as_str()).unwrap_or_else(|err| {
            eprintln!("{}", err);
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
    create_file: bool
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 
            
        let mut show_help = false;
        let mut show_version = false;
        let mut update_atime = false;
        let mut create_file = true;
        let mut file_name: Option<String> = None;

        for arg in args {
            if arg.starts_with("-") {
                match &arg[..] {
                    "--help" | "-h" => {
                        show_help = true;
                        create_file = false;
                        return Ok(Config { file_name, show_help, show_version, update_atime, create_file });
                    },
                    "--version" => {
                        show_version = true;
                        create_file = false;
                        return Ok(Config { file_name, show_help, show_version, update_atime, create_file });
                    },
                    "-a" => { 
                        update_atime = true;
                    },
                    "-c" => {
                        create_file = false;
                    },
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
            create_file
        })

    }

}
