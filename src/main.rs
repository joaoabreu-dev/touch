
use std::process;
use std::env;
use regex::Regex;

use touch::{ create_file, show_help, show_version };

fn main() {
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
   
    if !config.flags.is_none() {
        let flags = config.flags.unwrap();
        
        for flag in flags {
            match flag {
                Flags::Help => {
                    show_help();
                    process::exit(0);
                },
                Flags::Version => {
                    show_version();
                    process::exit(0);
                },
                _ => {}
            };
        }
    }

    create_file(config.file_name.unwrap().as_str()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

}

#[derive(Debug)]
enum Flags {
    Help,
    Version,
    A,
    C,
    D,
    F,
    M,
    R,
    T
}

#[derive(Debug)]
struct Config {
    file_name: Option<String>,
    flags: Option<Vec<Flags>>
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 

        let mut flags: Vec<Flags> = Vec::new();
        let mut file_name: Option<String> = None;

        for arg in args {
            if arg.starts_with("-") {
                let arg: Flags = match &arg[..] {
                    "--help" | "-h" => {
                       return Ok(Config { file_name: None, flags: Some(vec![Flags::Help]) }); 
                    },
                    "--version" => {
                       return Ok(Config { file_name: None, flags: Some(vec![Flags::Version]) }); 
                    },
                    "-a" => Flags::A,
                    "-c" => Flags::C,
                    "-d" => Flags::D,
                    "-f" => Flags::F,
                    "-m" => Flags::M,
                    "-r" => Flags::R,
                    "-t" => Flags::T,
                    _ => return Err("Flag inválida!"), 
                };
                flags.push(arg);    
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
        
        if flags.len() > 0 {
            Ok(Config { file_name, flags: Some(flags) })
        } else {
            Ok(Config { file_name, flags: None })
        }

    }

}
