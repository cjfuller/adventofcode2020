use crate::util;
use regex::Regex;
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Instruction {
    Acc(i64),
    Nop(i64),
    Jmp(i64),
}

use Instruction::*;

struct MachineState {
    instruction_pointer: usize,
    accumulator: i64,
}

fn parse_input() -> Vec<Instruction> {
    let re = Regex::new(r"(?P<instruction_code>\w{3}) (?P<num>[\d+-]+)").unwrap();
    util::file_lines("./data/d8.txt")
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            match &caps["instruction_code"] {
                "nop" => Nop(caps["num"].parse::<i64>().unwrap()),
                "acc" => Acc(caps["num"].parse::<i64>().unwrap()),
                "jmp" => Jmp(caps["num"].parse::<i64>().unwrap()),
                _ => panic!("Unknown instruction {}", &caps["instruction_code"]),
            }
        })
        .collect()
}

fn terminal_state(instructions: &[Instruction]) -> MachineState {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut state = MachineState {
        instruction_pointer: 0,
        accumulator: 0,
    };
    while state.instruction_pointer < instructions.len()
        && !visited.contains(&state.instruction_pointer)
    {
        visited.insert(state.instruction_pointer);
        state = match instructions[state.instruction_pointer] {
            Acc(amount) => MachineState {
                instruction_pointer: state.instruction_pointer + 1,
                accumulator: state.accumulator + amount,
            },
            Nop(_) => MachineState {
                instruction_pointer: state.instruction_pointer + 1,
                accumulator: state.accumulator,
            },
            Jmp(offset) => MachineState {
                instruction_pointer: (state.instruction_pointer as i64 + offset) as usize,
                accumulator: state.accumulator,
            },
        }
    }
    state
}

#[allow(dead_code)]
pub fn solve() {
    println!("Part 1:");
    let instructions = parse_input();
    let state = terminal_state(&instructions);
    println!("{}", state.accumulator);

    println!("Part 2:");
    for i in 0..instructions.len() {
        match instructions[i] {
            Acc(_) => (),
            Nop(amt) => {
                let mut my_instructions = instructions.clone();
                my_instructions[i] = Jmp(amt);
                let state = terminal_state(&my_instructions);
                if state.instruction_pointer == instructions.len() {
                    println!("{}", state.accumulator);
                    return;
                }
            }
            Jmp(amt) => {
                let mut my_instructions = instructions.clone();
                my_instructions[i] = Nop(amt);
                let state = terminal_state(&my_instructions);
                if state.instruction_pointer == instructions.len() {
                    println!("{}", state.accumulator);
                    return;
                }
            }
        }
    }
}
