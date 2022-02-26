use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

fn readlines<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().map(|line| line.unwrap()).collect()
}

/// Return the content of the file specified by the given path.
/// If `None` is passed, return the content of stdin.
pub fn scan(opt_path: &Option<String>) -> Vec<String> {
    match opt_path {
        Some(path) => {
            let f = File::open(path).unwrap();
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
