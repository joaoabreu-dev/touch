
use std::fs;
use std::process;
use std::env;
use regex::Regex;

use touch::{ create_file };

fn main() {
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
   
    println!("config: {:?}", config);

    create_file(config.file_name.as_str()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

}

#[derive(Debug)]
enum Flags {
    H, 
    I,
    P,
    F,
    K,
}

#[derive(Debug)]
struct Config {
    file_name: String,
    flags: Option<Vec<Flags>>
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 

        let mut flags: Vec<Flags> = Vec::new();
        let mut file_name: Option<String> = None;

        for arg in args {
            if arg.starts_with("-") {
                println!("{}", &arg[1..]);
                let arg = match &arg[1..] {
                    "H" => Some(Flags::H),
                    "F" => Some(Flags::F),
                    "I" => Some(Flags::I),
                    "K" => Some(Flags::K),
                    _ => None
                };
                flags.push(arg.ok_or("Flag inválida!")?);    
            } else {
                file_name = Some(arg);
            }
        }
       
        if file_name.is_none() {
            return Err("Indique um nome para o ficheiro");
        }
    
        let file_name: String = file_name.unwrap();

        let regex = Regex::new("[<>:\"/\\\\|?*]").unwrap();

        if regex.is_match(&file_name) {
            return Err("Caracteres inválidos no nome do ficheiro!");
        }
        
        if flags.len() > 0 {
            Ok(Config { file_name, flags: Some(flags) })
        } else {
            Ok(Config { file_name, flags: None })
        }

    }

}
