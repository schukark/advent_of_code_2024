use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
};

use pest::Parser;
use pest_derive::Parser;
use tqdm::tqdm;

use crate::utils::input::read_file;

#[derive(Parser)]
#[grammar = "config_files/day17.pest"]
struct AssemblerParser;

#[derive(Debug, Clone)]
struct State {
    registers: [u64; 3],
    instruction_stack: InstructionStack,
    out: Vec<u64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operand {
    Literal(u64),
    Register(char),
}

type Instruction = (u64, Operand);
#[derive(Debug, Clone)]
struct InstructionStack {
    instructions: Vec<Instruction>,
    pointer: usize,
}

impl InstructionStack {
    fn new() -> InstructionStack {
        InstructionStack {
            instructions: Vec::new(),
            pointer: 0,
        }
    }

    fn proceed(&mut self) -> Result<(), Box<dyn Error>> {
        self.pointer += 1;

        if self.pointer >= self.instructions.len() {
            return Err("stack overflow".into());
        }

        Ok(())
    }

    fn jump(&mut self, jump_to: usize) -> Result<(), Box<dyn Error>> {
        self.pointer = jump_to;

        if self.pointer >= self.instructions.len() {
            return Err("stack overflow".into());
        }

        Ok(())
    }
}

impl State {
    fn new() -> Self {
        State {
            registers: [0; 3],
            instruction_stack: InstructionStack::new(),
            out: Vec::new(),
        }
    }

    fn get_combo_value(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Literal(lit) if (0..=3).contains(&lit) => lit,
            Operand::Literal(reg) if (4..=6).contains(&reg) => self.registers[reg as usize - 4],
            Operand::Register(reg) if ('A'..='C').contains(&reg) => {
                self.registers[(reg as u8 - b'A') as usize]
            }
            _ => panic!("Invalid register"),
        }
    }

    fn get_literal_value(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Literal(lit) => lit,
            _ => panic!("Invalid literal"),
        }
    }

    fn set_value(&mut self, register: Operand, value: u64) {
        match register {
            Operand::Register(reg) if ('A'..='C').contains(&reg) => {
                self.registers[(reg as u8 - b'A') as usize] = value;
            }
            _ => panic!("Invalid register"),
        }
    }

    fn execute_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        let (op, operand) = self.instruction_stack.instructions[self.instruction_stack.pointer];

        match op {
            0 => {
                let numerator = self.get_combo_value(Operand::Register('A'));
                let denominator = 2_u64.pow(self.get_combo_value(operand) as u32);
                self.set_value(Operand::Register('A'), numerator / denominator);
                self.instruction_stack.proceed()
            }
            1 => {
                let b_register = self.get_combo_value(Operand::Register('B'));
                let b_value = b_register ^ self.get_literal_value(operand);
                self.set_value(Operand::Register('B'), b_value);
                self.instruction_stack.proceed()
            }
            2 => {
                let b_register = self.get_combo_value(operand);
                let b_value = b_register % 8;
                self.set_value(Operand::Register('B'), b_value);
                self.instruction_stack.proceed()
            }
            3 => {
                let a_register = self.get_combo_value(Operand::Register('A'));
                if a_register == 0 {
                    return self.instruction_stack.proceed();
                }
                let jump_to = self.get_literal_value(operand);
                self.instruction_stack.jump(jump_to as usize)
            }
            4 => {
                let b_register = self.get_combo_value(Operand::Register('B'));
                let c_register = self.get_combo_value(Operand::Register('C'));
                let b_value = b_register ^ c_register;
                self.set_value(Operand::Register('B'), b_value);
                self.instruction_stack.proceed()
            }
            5 => {
                let value = self.get_combo_value(operand) % 8;
                self.out.push(value);
                self.instruction_stack.proceed()
            }
            6 => {
                let numerator = self.get_combo_value(Operand::Register('A'));
                let denominator = 2_u64.pow(self.get_combo_value(operand) as u32);
                self.set_value(Operand::Register('B'), numerator / denominator);
                self.instruction_stack.proceed()
            }
            7 => {
                let numerator = self.get_combo_value(Operand::Register('A'));
                let denominator = 2_u64.pow(self.get_combo_value(operand) as u32);
                self.set_value(Operand::Register('C'), numerator / denominator);
                self.instruction_stack.proceed()
            }
            _ => panic!("Invalid operand"),
        }
    }

    fn execute_program(&mut self) {
        while self.execute_instruction().is_ok() {}
    }

    fn get_out(&self) -> Vec<u64> {
        self.out.clone()
    }
}

fn parse_input(input: &str) -> Result<State, Box<dyn Error>> {
    let pairs = AssemblerParser::parse(Rule::file, input)?.next().unwrap();

    let mut state = State::new();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::program => {
                for rule in pair.into_inner() {
                    match rule.as_rule() {
                        Rule::instruction => {
                            let mut instruction_pair = rule.into_inner();
                            let op = instruction_pair.next().unwrap().as_str().parse::<u64>()?;
                            let operand = instruction_pair.next().unwrap();

                            let operand_value = match operand.as_rule() {
                                Rule::value => Operand::Literal(operand.as_str().parse::<u64>()?),
                                Rule::register => {
                                    Operand::Register(operand.as_str().chars().next().unwrap())
                                }
                                _ => unreachable!(),
                            };

                            state
                                .instruction_stack
                                .instructions
                                .push((op, operand_value));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::register_line => {
                let mut register_pair = pair.into_inner();
                let register = register_pair
                    .next()
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();
                let value = register_pair.next().unwrap().as_str().parse::<u64>()?;

                state.set_value(Operand::Register(register), value);
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(state)
}

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day17.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let mut state = parse_input(input).expect("Invalid input");
    state.execute_program();

    println!(
        "{}",
        state
            .get_out()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut state = State::new();
        state.set_value(Operand::Register('C'), 9);
        state.instruction_stack.instructions = vec![(2, Operand::Literal(6))];
        state.execute_program();

        assert_eq!(state.get_combo_value(Operand::Register('C')), 9);
    }

    #[test]
    fn test_example2() {
        let mut state = State::new();
        state.set_value(Operand::Register('A'), 10);
        state.instruction_stack.instructions = vec![
            (5, Operand::Literal(0)),
            (5, Operand::Literal(1)),
            (5, Operand::Literal(4)),
        ];

        state.execute_program();

        assert_eq!(state.get_out(), vec![0, 1, 2]);
    }

    #[test]
    fn test_example3() {
        let mut state = State::new();
        state.set_value(Operand::Register('A'), 2024);
        state.instruction_stack.instructions = vec![
            (0, Operand::Literal(1)),
            (5, Operand::Literal(4)),
            (3, Operand::Literal(0)),
        ];

        state.execute_program();

        assert_eq!(state.get_out(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.get_combo_value(Operand::Register('A')), 0);
    }

    #[test]
    fn test_example4() {
        let mut state = State::new();
        state.set_value(Operand::Register('B'), 29);
        state.instruction_stack.instructions = vec![(1, Operand::Literal(7))];
        state.execute_program();

        assert_eq!(state.get_combo_value(Operand::Register('B')), 26);
    }

    #[test]
    fn test_example5() {
        let mut state = State::new();
        state.set_value(Operand::Register('C'), 43690);
        state.set_value(Operand::Register('B'), 2024);
        state.instruction_stack.instructions = vec![(4, Operand::Literal(0))];
        state.execute_program();

        assert_eq!(state.get_combo_value(Operand::Register('B')), 44354);
    }

    #[test]
    fn test_parse_example() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

        let state = parse_input(input).expect("Invalid input");

        assert_eq!(state.registers[0], 729);
        assert_eq!(state.registers[1], 0);
        assert_eq!(state.registers[2], 0);

        assert_eq!(
            state.instruction_stack.instructions,
            vec![
                (0, Operand::Literal(1)),
                (5, Operand::Literal(4)),
                (3, Operand::Literal(0)),
            ]
        );
    }
}
