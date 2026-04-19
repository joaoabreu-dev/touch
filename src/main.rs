
use std::fs;
use std::process;
use std::env;
use regex::Regex;

use touch::{ create_file };

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 2 {
            return Err("Número de parametros em inválido!");
        }
    
        let file_name = args[1].clone();
        
        let regex = Regex::new("[<>:\"/\\\\|?*]").unwrap();

        if regex.is_match(&file_name) {
            return Err("Caracteres inválidos no nome do ficheiro!");
        }

        Ok(Config { file_name })
    }
}
