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
    // Solution for part 1
}

fn part2(input: &str) {
    // Solution for part 2
}
