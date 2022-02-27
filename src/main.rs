mod args;
mod calculator;
mod parser;
mod scanner;

use args::Args;
use calculator::calc;
use clap::StructOpt;
use log::info;
use scanner::scan;
use std::env;

fn main() {
    let opts = Args::parse();
    if opts.verbose {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    match &opts.filename {
        Some(filename) => info!("File specified: {}", filename),
        None => info!("No file specified."),
    }

    let lines = scan(&opts.filename);
    for line in lines {
        println!("Input:  {}", line);

        let tokens = parser::parse(&line);
        info!("Tokens: {:?}", tokens);

        let result = calc(&tokens);
        println!("Result: {}", result);
    }
}
