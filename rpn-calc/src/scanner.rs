use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

fn readlines<R: BufRead>(reader: R) -> Result<Vec<String>> {
    reader
        .lines()
        .map(|line| line.context("Failed to read inputs"))
        .collect()
}

/// Return the content of the file specified by the given path.
/// If `None` is passed, return the content of stdin.
pub fn scan(opt_path: &Option<PathBuf>) -> Result<Vec<String>> {
    match opt_path {
        Some(path) => {
            let f = File::open(path).with_context(|| format!("Failed to open file {:?}", path))?;
            let reader = BufReader::new(f);
            readlines(reader)
        }
        None => {
            let stdin = stdin();
            let reader = stdin.lock();
            readlines(reader)
        }
    }
}
