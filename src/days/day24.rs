use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day24.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    
}

fn part2(input: &str) {

}