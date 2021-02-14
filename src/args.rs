use gumdrop::Options;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Evaluate code", meta = "[EXPR]")]
    pub eval: Option<String>,
    #[options(help = "Evaluate script", meta = "[FILE]")]
    pub file: Option<String>,
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

pub fn get_script() -> String {
    let args = Args::parse_args_default_or_exit();

    if args.eval.is_none() && args.file.is_none() {
        println!("{}", Args::usage());
        std::process::exit(0);
    }

    if args.eval.is_some() {
        args.eval.unwrap()
    } else {
        match read_file(&args.file.unwrap()) {
            Ok(script) => script,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0);
            },
        }
    }
}
