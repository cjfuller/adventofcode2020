use crate::util;
use std::collections::HashMap;
use std::vec::Vec;

struct Group {
    group_size: i32,
    answer_counts: HashMap<char, i32>,
}

fn parse_input() -> Vec<Group> {
    util::partition_by_blank_lines(util::file_lines("./data/d6.txt"))
        .iter()
        .map(|lines| {
            let mut group_size: i32 = 0;
            let mut result = HashMap::new();
            lines.iter().for_each(|line| {
                group_size += 1;
                line.chars().for_each(|ch| {
                    result.insert(ch, result.get(&ch).unwrap_or(&0) + 1);
                })
            });
            Group {
                group_size,
                answer_counts: result,
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve() {
    println!("Part 1:");
    println!(
        "{}",
        parse_input()
            .iter()
            .map(|group| group.answer_counts.len())
            .sum::<usize>()
    );
    println!("Part 2:");
    println!(
        "{}",
        parse_input()
            .iter()
            .map(|group| group
                .answer_counts
                .values()
                .filter(|v| **v == group.group_size)
                .count())
            .sum::<usize>()
    )
}
