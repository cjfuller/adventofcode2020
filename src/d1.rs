use std::vec::Vec;

use crate::util;

fn read_numbers() -> Vec<i64> {
    util::file_lines("./data/d1p1.txt")
        .iter()
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

fn brute_force_sum_two(numbers: &[i64], to: i64) -> Option<(i64, i64)> {
    for num0 in numbers {
        for num1 in numbers {
            if num0 + num1 == to {
                return Some((*num0, *num1));
            }
        }
    }
    None
}

fn brute_force_sum_three(numbers: &[i64], to: i64) -> Option<(i64, i64, i64)> {
    for num0 in numbers {
        for num1 in numbers {
            for num2 in numbers {
                if num0 + num1 + num2 == to {
                    return Some((*num0, *num1, *num2));
                }
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn solvep1() {
    let nums = read_numbers();
    let (n0, n1) = brute_force_sum_two(&nums, 2020).unwrap();
    println!("{}", n0 * n1)
}

#[allow(dead_code)]
pub fn solvep2() {
    let nums = read_numbers();
    let (n0, n1, n2) = brute_force_sum_three(&nums, 2020).unwrap();
    println!("{}", n0 * n1 * n2)
}
