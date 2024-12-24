use std::{env, time::Instant};
mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <day> <part>");
        return;
    }

    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day");
            return;
        }
    };

    let part: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid part");
            return;
        }
    };

    let before = Instant::now();

    match day {
        1 => days::day01::run(part),
        2 => days::day02::run(part),
        3 => days::day03::run(part),
        4 => days::day04::run(part),
        5 => days::day05::run(part),
        6 => days::day06::run(part),
        7 => days::day07::run(part),
        8 => days::day08::run(part),
        9 => days::day09::run(part),
        10 => days::day10::run(part),
        11 => days::day11::run(part),
        12 => days::day12::run(part),
        13 => days::day13::run(part),
        14 => days::day14::run(part),
        15 => days::day15::run(part),
        _ => println!("Invalid day"),
    }

    println!("Execution time: {:.2?}", before.elapsed());
}
