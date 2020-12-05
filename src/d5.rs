use crate::util;

fn parse_seat(seat_str: &str) -> (i32, i32) {
    let row = seat_str
        .chars()
        .take(7)
        .enumerate()
        .fold(0, |acc, (i, curr)| {
            acc + ((if curr == 'B' { 1 } else { 0 }) << (6 - i))
        });

    let col = seat_str
        .chars()
        .skip(7)
        .enumerate()
        .fold(0, |acc, (i, curr)| {
            acc + ((if curr == 'R' { 1 } else { 0 }) << (2 - i))
        });
    (row, col)
}

fn seat_id(r: i32, c: i32) -> i32 {
    8 * r + c
}

#[cfg(test)]
mod test {
    use crate::d5::*;
    #[test]
    fn test_rc() {
        assert_eq!(parse_seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(parse_seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(parse_seat("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_id() {
        assert_eq!(seat_id(70, 7), 567);
        assert_eq!(seat_id(14, 7), 119);
        assert_eq!(seat_id(102, 4), 820);
    }
}

#[allow(dead_code)]
pub fn solve() {
    let input = util::file_lines("./data/d5.txt");
    let max_id = input.iter().fold(0, |acc, seat| {
        let (row, col) = parse_seat(seat);
        let id = seat_id(row, col);
        if acc > id {
            acc
        } else {
            id
        }
    });
    println!("Part 1:");
    println!("{}", max_id);

    println!("Part 2:");
    let mut ids: std::vec::Vec<i32> = input
        .iter()
        .map(|seat| {
            let (r, c) = parse_seat(seat);
            seat_id(r, c)
        })
        .collect();
    ids.sort();
    ids.iter().skip(1).zip(ids.iter()).for_each(|(l, r)| {
        if *l != r + 1 {
            println!("{}", r + 1)
        }
    });
}
