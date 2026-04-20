
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
    
    create_file(config.file_name.as_str()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

}

struct Config {
    file_name: String
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Por favor indique um nome para o ficheiro!"),
        };

        let regex = Regex::new("[<>:\"/\\\\|?*]").unwrap();

        if regex.is_match(&file_name) {
            return Err("Caracteres inválidos no nome do ficheiro!");
        }

        Ok(Config { file_name })
    }
}
