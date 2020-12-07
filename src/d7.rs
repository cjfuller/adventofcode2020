use crate::util;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

fn parse_part(part: &str) -> Option<(String, i32)> {
    if part.contains("no other bags") {
        None
    } else {
        let re = Regex::new(r"^(?P<number>\d+) (?P<color>.*) bags?$").unwrap();
        Some(
            re.captures(part)
                .map(|caps| {
                    (
                        caps["color"].to_string(),
                        caps["number"].parse::<i32>().unwrap(),
                    )
                })
                .unwrap(),
        )
    }
}

fn parse_contents(contents: &str) -> Vec<(String, i32)> {
    let parts = contents.split(", ");
    parts
        .map(|part| parse_part(part))
        .filter_map(|p| p)
        .collect()
}

fn parse_rule(rule: &str) -> (String, Vec<(String, i32)>) {
    let re = Regex::new(r"^(?P<source_color>.*) bags contain (?P<contents>.*)\.$").unwrap();
    let caps = re.captures(rule).unwrap();
    (
        caps["source_color"].to_string(),
        parse_contents(&caps["contents"]),
    )
}

fn parse_input() -> HashMap<String, Vec<(String, i32)>> {
    util::file_lines("./data/d7.txt")
        .iter()
        .map(|line| parse_rule(line))
        .collect()
}

#[allow(dead_code)]
pub fn solve() {
    println!("Part 1:");
    let rules = parse_input();
    let mut inversion_parts: Vec<(String, String)> = rules
        .iter()
        .flat_map(|(k, v)| v.iter().map(move |(color, _)| (color.clone(), k.clone())))
        .collect();
    inversion_parts.sort_by_key(|(contained, _)| contained.clone());
    let mut inversion: HashMap<String, Vec<String>> = HashMap::new();
    inversion_parts.iter().for_each(|(contained, container)| {
        if !inversion.contains_key(contained) {
            inversion.insert(contained.clone(), vec![]);
        }
        let v = inversion.get_mut(contained).unwrap();
        v.push(container.clone());
    });
    let my_color = "shiny gold";

    let mut last_size: usize = 0;
    let mut tracking: HashSet<String> = HashSet::new();
    inversion[my_color].iter().for_each(|c| {
        tracking.insert(c.clone());
    });
    while tracking.len() != last_size {
        last_size = tracking.len();
        let mut next_tracking: HashSet<String> = tracking.clone();
        tracking.iter().for_each(|c_curr| {
            inversion
                .get(c_curr)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|c_next| {
                    next_tracking.insert(c_next.clone());
                })
        });
        tracking = next_tracking;
    }
    println!("{}", tracking.len());

    println!("Part 2:");
    let mut total_bags = -1;
    let mut curr = vec![("shiny gold".to_string(), 1)];
    while !curr.is_empty() {
        let next_bags = curr.iter().flat_map(|(color, number)| {
            total_bags += number;
            rules[color].iter().map(move |(inner_color, inner_number)| {
                (inner_color.clone(), inner_number * number)
            })
        });
        curr = next_bags.collect();
    }
    println!("{}", total_bags);
}
