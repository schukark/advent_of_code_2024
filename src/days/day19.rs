use std::collections::{BTreeSet, HashMap, HashSet};

use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day19.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let towels = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect::<HashSet<_>>();

    let max_len = towels
        .iter()
        .fold(usize::MIN, |acc, elem| usize::max(acc, elem.len()));

    println!(
        "{}",
        input
            .lines()
            .skip(2)
            .filter(|line| can_form(line, &towels, max_len) > 0)
            .count()
    );
}

fn can_form(stripes: &str, towels: &HashSet<&str>, max_len: usize) -> usize {
    let mut dp = vec![0; stripes.len() + 1];
    dp[0] = 1;

    for i in 1..=stripes.len() {
        for j in 0..i {
            if j + max_len < i {
                continue;
            }
            let substr = &stripes[j..i];
            if towels.contains(substr) {
                dp[i] += dp[j];
            }
        }
    }

    dp[stripes.len()]
}

fn part2(input: &str) {
    let towels = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect::<HashSet<_>>();

    let max_len = towels
        .iter()
        .fold(usize::MIN, |acc, elem| usize::max(acc, elem.len()));

    println!(
        "{}",
        input
            .lines()
            .skip(2)
            .map(|line| can_form(line, &towels, max_len))
            .sum::<usize>()
    );
}
