use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, time::Instant};
extern crate reqwest;
use dotenv::dotenv;
use reqwest::header;

mod days;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "create" {
        let day = args[2].parse::<u32>().unwrap();
        fs::write(format!("src/days/day{}.rs", day), get_template(day))
            .expect("Unable to write to file");

        let mut mod_file = fs::OpenOptions::new()
            .append(true)
            .open("src/days/mod.rs")
            .unwrap();

        if let Err(e) = writeln!(mod_file, "pub mod day{};", day) {
            eprintln!("Couldn't write to file: {}", e);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            [
                "session=",
                &env::var("AOC_SESSION").unwrap_or("".to_string()),
            ]
            .concat()
            .parse()
            .unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let res = client
            .get(
                [
                    "https://adventofcode.com/2024/day/",
                    &day.to_string(),
                    "/input",
                ]
                .concat(),
            )
            .headers(headers)
            .send()?
            .text()?;

        fs::write(format!("inputs/day{}.txt", day), res).expect("Unable to write to file");

        return Ok(());
    }

    if args.len() != 3 {
        return Err("Usage: cargo run <day> <part>".into());
    }

    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Invalid day".into());
        }
    };

    let part: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Invalid part".into());
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
        16 => days::day16::run(part),
        17 => days::day17::run(part),
        18 => days::day18::run(part),
        19 => days::day19::run(part),
        20 => days::day20::run(part),
        21 => days::day21::run(part),
        22 => days::day22::run(part),
        23 => days::day23::run(part),
        24 => days::day24::run(part),
        25 => days::day25::run(part),
        _ => println!("Invalid day"),
    }

    println!("Execution time: {:.2?}", before.elapsed());

    Ok(())
}

fn get_template(day: u32) -> String {
    format!(
        "use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {{
    let input = read_file(\"inputs/day{}.txt\");

    match part {{
        1 => part1(&input),
        2 => part2(&input),
        _ => println!(\"Invalid part\"),
    }}
}}

fn part1(input: &str) {{
    
}}

fn part2(input: &str) {{

}}",
        day
    )
}
