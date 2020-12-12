use crate::util;
use std::collections::VecDeque;

fn sums_from(num: i64, factors: &VecDeque<i64>) -> bool {
    for i in 0..factors.len() {
        for j in (i + 1)..factors.len() {
            if num == factors[i] + factors[j] {
                return true;
            }
        }
    }
    false
}

fn solvep1(curr_queue: &mut VecDeque<i64>, remaining_elements: &mut VecDeque<i64>) -> i64 {
    let next_elt = remaining_elements.pop_front().unwrap();
    if sums_from(next_elt, curr_queue) {
        curr_queue.pop_front();
        curr_queue.push_back(next_elt);
    } else {
        return next_elt;
    }
    solvep1(curr_queue, remaining_elements)
}

fn find_contiguous_sum(nums: &mut VecDeque<i64>, sum_to: i64) -> (i64, i64) {
    let mut min_in_sum = i64::MAX;
    let mut max_in_sum = i64::MIN;
    let mut sum = 0;
    if nums[0] == sum_to {
        nums.pop_front();
    }
    for i in nums.iter() {
        sum += *i;
        if *i < min_in_sum {
            min_in_sum = *i
        }
        if *i > max_in_sum {
            max_in_sum = *i
        }
        if sum >= sum_to {
            break;
        }
    }
    if sum == sum_to {
        return (min_in_sum, max_in_sum);
    }
    nums.pop_front().unwrap();
    find_contiguous_sum(nums, sum_to)
}

#[allow(dead_code)]
pub fn solve() {
    let input = util::file_lines("./data/d9.txt");
    let mut current_elements: VecDeque<i64> = VecDeque::with_capacity(25);
    let mut remaining_elements: VecDeque<i64> = VecDeque::new();
    input.iter().take(25).for_each(|e| {
        current_elements.push_back(e.parse::<i64>().unwrap());
    });
    input.iter().skip(25).for_each(|e| {
        remaining_elements.push_back(e.parse::<i64>().unwrap());
    });

    println!("Part 1:");
    let invalid_num = solvep1(&mut current_elements, &mut remaining_elements);
    println!("{}", invalid_num);
    println!("Part 2:");
    let mut input_nums: VecDeque<i64> = util::file_lines("./data/d9.txt")
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let (min, max) = find_contiguous_sum(&mut input_nums, invalid_num);
    println!("{}", min + max);
}
