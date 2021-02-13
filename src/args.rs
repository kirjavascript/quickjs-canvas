use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Evaluate code", meta = "[EXPR]")]
    pub eval: Option<String>,
    #[options(help = "Evaluate script", meta = "[FILE]")]
    pub file: Option<String>,
}

pub fn get() -> Args {
    let args = Args::parse_args_default_or_exit();

    if args.eval.is_none() || !args.file.is_none() {
        println!("{}", Args::usage());
        std::process::exit(0);
    }

    args
}
