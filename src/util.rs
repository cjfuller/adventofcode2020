use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

pub fn file_lines(filename: &str) -> Vec<String> {
    io::BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|s| s.unwrap())
        .collect()
}

pub fn partition_by_blank_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut partitioned = vec![vec![]];

    lines.iter().for_each(|line| {
        if line == "" {
            partitioned.push(vec![]);
        } else {
            let last = partitioned.last_mut().unwrap();
            last.push(line.clone());
        }
    });
    partitioned
}
