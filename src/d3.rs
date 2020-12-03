use crate::util;
use std::vec::Vec;

fn parse_input() -> Vec<Vec<bool>> {
    util::file_lines("./data/d3.txt")
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Got an unexpected character"),
                })
                .collect()
        })
        .collect()
}

fn count_along_slope_with_state(
    grid: &[Vec<bool>],
    horiz_step: usize,
    vert_step: usize,
    horiz_pos: usize,
    vert_pos: usize,
) -> i64 {
    if vert_pos >= grid.len() {
        return 0;
    }
    let curr_val = {
        let curr_row = &grid[vert_pos];
        if curr_row[horiz_pos % curr_row.len()] {
            1
        } else {
            0
        }
    };
    curr_val
        + count_along_slope_with_state(
            grid,
            horiz_step,
            vert_step,
            horiz_pos + horiz_step,
            vert_pos + vert_step,
        )
}

fn count_along_slope(grid: &[Vec<bool>], horiz_step: usize, vert_step: usize) -> i64 {
    count_along_slope_with_state(grid, horiz_step, vert_step, 0, 0)
}

pub fn solve() {
    println!("Part 1");
    let input = parse_input();
    println!("{}", count_along_slope(&input, 3, 1));
    println!("Part 2");
    println!(
        "{}",
        count_along_slope(&input, 3, 1)
            * count_along_slope(&input, 1, 1)
            * count_along_slope(&input, 5, 1)
            * count_along_slope(&input, 7, 1)
            * count_along_slope(&input, 1, 2)
    )
}
