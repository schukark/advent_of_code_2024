use std::collections::{HashMap, HashSet};

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day08.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let char_to_antenna_map = input.lines().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<char, Vec<(i32, i32)>>, (idx, line)| {
            line.chars().enumerate().for_each(|(idx_line, c)| {
                if c == '.' {
                    return;
                }

                if let Some(vec) = acc.get_mut(&c) {
                    vec.push((idx as i32, idx_line as i32));
                } else {
                    acc.insert(c, vec![(idx as i32, idx_line as i32)]);
                }
            });

            acc
        },
    );

    let mut unique_points = HashSet::new();

    for letter in char_to_antenna_map.keys() {
        for p1 in char_to_antenna_map.get(letter).unwrap() {
            for p2 in char_to_antenna_map.get(letter).unwrap() {
                if p1 == p2 {
                    continue;
                }

                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;

                // P1 - P2 - P_new
                let p_new = (p2.0 + dx, p2.1 + dy);

                if p_new.0 >= 0
                    && p_new.1 >= 0
                    && p_new.0 < grid.len() as i32
                    && p_new.1 < grid[0].len() as i32
                {
                    unique_points.insert(p_new);
                }
            }
        }
    }
    println!("{:#?}", unique_points.len());
}

fn part2(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let char_to_antenna_map = input.lines().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<char, Vec<(i32, i32)>>, (idx, line)| {
            line.chars().enumerate().for_each(|(idx_line, c)| {
                if c == '.' {
                    return;
                }

                if let Some(vec) = acc.get_mut(&c) {
                    vec.push((idx as i32, idx_line as i32));
                } else {
                    acc.insert(c, vec![(idx as i32, idx_line as i32)]);
                }
            });

            acc
        },
    );

    let mut unique_points = HashSet::new();

    for letter in char_to_antenna_map.keys() {
        for p1 in char_to_antenna_map.get(letter).unwrap() {
            for p2 in char_to_antenna_map.get(letter).unwrap() {
                if p1 == p2 {
                    continue;
                }

                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;

                let gcd = gcd::binary_u32(i32::abs(dx) as u32, i32::abs(dy) as u32) as i32;

                let dx = dx / gcd;
                let dy = dy / gcd;

                unique_points.insert(*p1);
                unique_points.insert(*p2);

                for step in 1.. {
                    let p_new = (p2.0 + dx * step, p2.1 + dy * step);

                    if p_new.0 < 0
                        || p_new.1 < 0
                        || p_new.0 >= grid.len() as i32
                        || p_new.1 >= grid[0].len() as i32
                    {
                        break;
                    }
                    unique_points.insert(p_new);
                }

                for step in 1.. {
                    let p_new = (p2.0 - dx * step, p2.1 - dy * step);

                    if p_new.0 < 0
                        || p_new.1 < 0
                        || p_new.0 >= grid.len() as i32
                        || p_new.1 >= grid[0].len() as i32
                    {
                        break;
                    }
                    unique_points.insert(p_new);
                }
            }
        }
    }
    println!("{:#?}", unique_points.len());
}
