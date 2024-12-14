use std::collections::HashSet;

use pest::Parser;
use pest_derive::Parser;

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day14.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

#[derive(Parser)]
#[grammar = "config_files/day14.pest"]
struct RobotParser {}

fn part1(input: &str) {
    const STEPS: isize = 100;

    let parse = RobotParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut quadrants: [usize; 4] = [0; 4];

    for pair in parse.into_inner() {
        let mut velocity = (0, 0);
        let mut position = (0, 0);

        for pair_inner in pair.into_inner() {
            match pair_inner.as_rule() {
                Rule::velocity => {
                    let mut inner = pair_inner.into_inner();
                    velocity.1 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                    velocity.0 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                }
                Rule::position => {
                    let mut inner = pair_inner.into_inner();
                    position.1 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                    position.0 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        if position == velocity && velocity == (0, 0) {
            continue;
        }

        let new_position = (
            (position.0 + velocity.0 * STEPS).rem_euclid(HEIGHT),
            (position.1 + velocity.1 * STEPS).rem_euclid(WIDTH),
        );

        let mut numerical = 0;

        if new_position.0 == HEIGHT / 2 || new_position.1 == WIDTH / 2 {
            continue;
        }

        if new_position.0 > HEIGHT / 2 {
            numerical += 1;
        }
        if new_position.1 > WIDTH / 2 {
            numerical += 2;
        }

        // println!("{:#?}", new_position);

        quadrants[numerical] += 1;
    }

    println!("{:?}", quadrants.iter().product::<usize>());
}

fn draw_robots(robots: &[Robot], height: isize, width: isize) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for robot in robots {
        let (x, y) = robot.pos;
        if x >= 0 && x < height && y >= 0 && y < width {
            grid[x as usize][y as usize] = '#';
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    vec: (isize, isize),
}

impl Robot {
    fn new(pos: (isize, isize), vec: (isize, isize)) -> Self {
        Self { pos, vec }
    }

    fn advance(&mut self) {
        self.pos.0 += self.vec.0;
        self.pos.1 += self.vec.1;

        self.pos.0 = self.pos.0.rem_euclid(HEIGHT);
        self.pos.1 = self.pos.1.rem_euclid(WIDTH);
    }
}

const HEIGHT: isize = 103;
const WIDTH: isize = 101;

fn part2(input: &str) {
    let parse = RobotParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for pair in parse.into_inner() {
        let mut velocity = (0, 0);
        let mut position = (0, 0);

        for pair_inner in pair.into_inner() {
            match pair_inner.as_rule() {
                Rule::velocity => {
                    let mut inner = pair_inner.into_inner();
                    velocity.1 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                    velocity.0 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                }
                Rule::position => {
                    let mut inner = pair_inner.into_inner();
                    position.1 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                    position.0 = inner.next().unwrap().as_str().parse::<isize>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        if position == velocity && velocity == (0, 0) {
            continue;
        }

        robots.push(Robot::new(position, velocity));
    }

    let init_state = robots.clone();

    let mut best = (0, f64::MAX);

    for i in 0..WIDTH * HEIGHT {
        let heuristic_state = heuristic(&robots);
        if heuristic_state < best.1 {
            best = (i, heuristic_state);
        }

        for robot in robots.iter_mut() {
            robot.advance();
        }
    }

    let mut robots = init_state;

    for _i in 0..best.0 {
        for robot in robots.iter_mut() {
            robot.advance();
        }
    }

    draw_robots(&robots, HEIGHT, WIDTH);
    println!("{:#?}", best.0);
}

fn heuristic(robots: &[Robot]) -> f64 {
    let means = robots
        .iter()
        .map(|robot| robot.pos)
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    let means = (
        means.0 as f64 / robots.len() as f64,
        means.1 as f64 / robots.len() as f64,
    );

    let variance = robots
        .iter()
        .map(|robot| robot.pos)
        .fold((0.0, 0.0), |acc, e| {
            (
                acc.0 + (e.0 as f64 - means.0).powf(2.0),
                acc.1 + (e.1 as f64 - means.1).powf(2.0),
            )
        });

    variance.0 * variance.0 + variance.1 * variance.1
}
