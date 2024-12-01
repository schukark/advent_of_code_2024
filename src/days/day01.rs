use hashbag::HashBag;

use super::super::utils::input;

pub fn run(part: u32) {
    let input = input::read_file("inputs/day01.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let pairs = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .take(2)
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut firsts = pairs.iter().map(|p| p[0]).collect::<Vec<_>>();
    let mut seconds = pairs.iter().map(|p| p[1]).collect::<Vec<_>>();

    firsts.sort();
    seconds.sort();

    let result = firsts.iter().zip(seconds.iter()).map(|(x, y)| i64::abs(x - y)).sum::<i64>();

    println!("{result}");
}

fn part2(input: &str) {
    let pairs = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .take(2)
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut firsts = pairs.iter().map(|p| p[0]).collect::<Vec<_>>();
    let seconds = pairs.iter().map(|p| p[1]).collect::<HashBag<_>>();

    firsts.sort();

    let result = firsts.iter().map(|x| seconds.contains(x) as i64 * x).sum::<i64>();

    println!("{result}");
}
