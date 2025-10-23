use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{ search, search_case_insensitive };

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments: usage is <query> <file_path> or -C <query> <file_path>");
        }

        let mut args_offset = 1;
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        if &args[1] == "-C"{
            ignore_case = true;
            args_offset = 2;
        }

        if args.len() - args_offset < 2 {
            return Err("Not enough arguments: usage is <query> <file_path> or -C <query> <file_path>");
        }

        let query = args[args_offset].clone();
        let file_path = args[args_offset + 1].clone();
        

        Ok(Config { query, file_path, ignore_case })
    }
}
