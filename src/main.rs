mod args;
mod logic;
mod scanner;

use args::Args;
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
    for line in &lines {
        println!("Input:  {}", line);
        let result = logic::calc(line);
        println!("Result: {}", result);
    }
}
