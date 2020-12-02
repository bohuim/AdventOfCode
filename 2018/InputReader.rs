use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: &str) -> Vec<String>
{
    let file = File::open(Path::new(filename)).expect("file not found");
    BufReader::new(file).lines().filter_map(Result::ok).collect()
}

