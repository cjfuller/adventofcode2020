use crate::util;

struct Policy {
    lower: usize,
    upper: usize,
    letter: char,
}

impl Policy {
    fn is_pw_valid_p1(&self, pw: &str) -> bool {
        let num = pw.chars().filter(|c| *c == self.letter).count();
        num <= self.upper && num >= self.lower
    }
    fn is_pw_valid_p2(&self, pw: &str) -> bool {
        (pw.chars().nth(self.lower - 1).unwrap() == self.letter)
            ^ (pw.chars().nth(self.upper - 1).unwrap() == self.letter)
    }
    fn from_spec(spec: &str) -> Policy {
        let parts: Vec<&str> = spec.split(' ').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].len(), 1);
        let letter: char = parts[1].chars().next().unwrap();
        let bounds: Vec<usize> = parts[0]
            .split('-')
            .map(|part| part.parse::<usize>().unwrap())
            .collect();
        Policy {
            lower: bounds[0],
            upper: bounds[1],
            letter,
        }
    }
}

fn read_and_parse_input() -> Vec<(Policy, String)> {
    util::file_lines("./data/d2.txt")
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(':').map(|part| part.trim()).collect();
            assert_eq!(parts.len(), 2);
            let pw = parts[1];
            let policy = Policy::from_spec(parts[0]);
            (policy, String::from(pw))
        })
        .collect()
}

#[allow(dead_code)]
pub fn solvep1() {
    let num_valid = read_and_parse_input()
        .iter()
        .filter(|(policy, pw)| policy.is_pw_valid_p1(&pw))
        .count();
    println!("{}", num_valid);
}

#[allow(dead_code)]
pub fn solvep2() {
    let num_valid = read_and_parse_input()
        .iter()
        .filter(|(policy, pw)| policy.is_pw_valid_p2(&pw))
        .count();
    println!("{}", num_valid);
}
