mod args;
mod scanner;

use args::Args;
use clap::StructOpt;
use log::info;
use scanner::scan;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opts = Args::parse();
    match &opts.filename {
        Some(filename) => info!("File specified: {}", filename),
        None => info!("No file specified."),
    }

    let lines = scan(&opts.filename);
    for line in lines {
        println!("{}", line);
    }
}
