use std::collections::HashMap;

use crate::utils::input;
use tqdm::tqdm;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day11.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn make_step(current_step: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_result: HashMap<usize, usize> = HashMap::new();

    for (&stone, &count) in current_step.iter() {
        if stone == 0 {
            *new_result.entry(1).or_default() += count;
        } else if stone.to_string().len() % 2 == 0 {
            let stone_len = stone.to_string().len();
            let left_stone = stone.to_string()[0..stone_len / 2]
                .parse::<usize>()
                .unwrap();
            let right_stone = stone.to_string()[stone_len / 2..stone_len]
                .parse::<usize>()
                .unwrap();

            *new_result.entry(left_stone).or_default() += count;
            *new_result.entry(right_stone).or_default() += count;
        } else {
            *new_result.entry(stone * 2024).or_default() += count;
        }
    }

    new_result
}

fn part1(input: &str) {
    const STEPS: usize = 25;
    let initial_state: HashMap<usize, usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| (x, 1))
        .collect();

    let mut current_state = initial_state;

    for _ in tqdm(0..STEPS) {
        current_state = make_step(&current_state);
    }

    println!("Part 1: {}", current_state.values().sum::<usize>());
}

fn part2(input: &str) {
    const STEPS: usize = 75;
    let initial_state: HashMap<usize, usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| (x, 1))
        .collect();

    let mut current_state = initial_state;

    for _ in tqdm(0..STEPS) {
        current_state = make_step(&current_state);
    }

    println!("Part 1: {}", current_state.values().sum::<usize>());
}
