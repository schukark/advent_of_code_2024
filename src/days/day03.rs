use regex::Regex;

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day03.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((?<a>[[:digit:]]+),(?<b>[[:digit:]]+)\)").unwrap();
    let mut result = 0;

    for mul_match in re.captures_iter(input) {
        let a: i32 = mul_match.name("a").unwrap().as_str().parse().unwrap();
        let b: i32 = mul_match.name("b").unwrap().as_str().parse().unwrap();

        result += a * b;
    }

    println!("{}", result);
}

fn find_do_donts(input: &str) -> Vec<(usize, i32)> {
    let mut answer = Vec::new();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();

    for (i, cap) in re_do.captures_iter(input).enumerate() {
        answer.push((cap.get(0).unwrap().start(), 1));
    }

    for (i, cap) in re_dont.captures_iter(input).enumerate() {
        answer.push((cap.get(0).unwrap().start(), -1));
    }

    answer.sort();

    answer
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\((?<a>[[:digit:]]+),(?<b>[[:digit:]]+)\)").unwrap();
    let mut result = 0;

    let input = &("do()".to_owned() + input);

    let do_donts = find_do_donts(input);

    for mul_match in re.captures_iter(input) {
        let index = mul_match.get(0).unwrap().start();
        let last_index = do_donts.binary_search_by_key(&index, &|&(a, _b)| a);

        let last_index = last_index.unwrap_err() - 1;

        let a: i32 = mul_match.name("a").unwrap().as_str().parse().unwrap();
        let b: i32 = mul_match.name("b").unwrap().as_str().parse().unwrap();

        if do_donts[last_index].1 == 1 {
            result += a * b;
        }
    }

    println!("{}", result);
}
