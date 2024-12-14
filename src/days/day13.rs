use std::ops::Mul;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::utils::input;

#[derive(Parser)]
#[grammar = "config_files/day13.pest"]
struct MachineParser {}

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day13.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Invalid part number"),
    }
}

struct Mat2x2 {
    mat: [[isize; 2]; 2],
}

impl Mat2x2 {
    fn det(&self) -> isize {
        self.mat[0][0] * self.mat[1][1] - self.mat[0][1] * self.mat[1][0]
    }

    fn false_invert(&self) -> Self {
        Self {
            mat: [
                [self.mat[1][1], -self.mat[0][1]],
                [-self.mat[1][0], self.mat[0][0]],
            ],
        }
    }
}

impl Mul<isize> for Mat2x2 {
    type Output = Mat2x2;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            mat: [
                [self.mat[0][0] * rhs, self.mat[0][1] * rhs],
                [self.mat[1][0] * rhs, self.mat[1][1] * rhs],
            ],
        }
    }
}

impl Mul<(isize, isize)> for Mat2x2 {
    type Output = (isize, isize);

    fn mul(self, rhs: (isize, isize)) -> Self::Output {
        let (x, y) = rhs;
        let res_x = x * self.mat[0][0] + y * self.mat[1][0];
        let res_y = x * self.mat[0][1] + y * self.mat[1][1];
        (res_x, res_y)
    }
}

impl From<Vec<(isize, isize)>> for Mat2x2 {
    fn from(value: Vec<(isize, isize)>) -> Self {
        Mat2x2 {
            mat: [[value[0].0, value[0].1], [value[1].0, value[1].1]],
        }
    }
}

fn part1(input: &str) {
    let parse = MachineParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut result = 0;

    for block in parse.into_inner() {
        let (prize, move_mat) = match parse_block(block) {
            Some(value) => value,
            None => break,
        };

        if move_mat.det() == 0 {
            panic!();
        }

        let tmp_solution = move_mat.false_invert() * prize;

        if tmp_solution.0 % move_mat.det() != 0 || tmp_solution.1 % move_mat.det() != 0 {
            continue;
        }

        let solution = (
            tmp_solution.0 / move_mat.det(),
            tmp_solution.1 / move_mat.det(),
        );

        if solution.0 < 0 || solution.1 < 0 || solution.0 > 100 || solution.1 > 100 {
            continue;
        }

        result += 3 * solution.0 + solution.1;
    }

    println!("{}", result);
}

fn parse_block(block: Pair<'_, Rule>) -> Option<((isize, isize), Mat2x2)> {
    let mut moves = Vec::<(isize, isize)>::new();
    let mut prize = (0, 0);
    for rule in block.into_inner() {
        match rule.as_rule() {
            Rule::button_line => {
                let mut shift = (0, 0);
                for shift_rule in rule.into_inner() {
                    match shift_rule.as_rule() {
                        Rule::x_shift => {
                            shift.0 = shift_rule
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<isize>()
                                .unwrap();
                        }
                        Rule::y_shift => {
                            shift.1 = shift_rule
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<isize>()
                                .unwrap();
                        }
                        _ => unreachable!(),
                    }
                }

                moves.push(shift);
            }
            Rule::prize_line => {
                let mut inner = rule.into_inner();
                prize.0 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                prize.1 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    if moves.is_empty() {
        return None;
    }
    let move_mat: Mat2x2 = moves.into();
    Some((prize, move_mat))
}

fn part2(input: &str) {
    let parse = MachineParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut result = 0;

    for block in parse.into_inner() {
        let (prize, move_mat) = match parse_block(block) {
            Some(value) => value,
            None => break,
        };

        let prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);

        if move_mat.det() == 0 {
            panic!();
        }

        let tmp_solution = move_mat.false_invert() * prize;

        if tmp_solution.0 % move_mat.det() != 0 || tmp_solution.1 % move_mat.det() != 0 {
            continue;
        }

        let solution = (
            tmp_solution.0 / move_mat.det(),
            tmp_solution.1 / move_mat.det(),
        );

        if solution.0 < 0 || solution.1 < 0 {
            continue;
        }

        result += 3 * solution.0 + solution.1;
    }

    println!("{}", result);
}
