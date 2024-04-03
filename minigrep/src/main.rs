//run from command line with `cargo run -- query haystack file_path poem.txt`
use minigrep::Config;
use std::env; //environment module to get command line arguments
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let (query,filepath) = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

