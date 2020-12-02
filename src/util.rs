use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

pub fn file_lines(filename: &str) -> Vec<String> {
    io::BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|s| s.unwrap())
        .collect()
}
