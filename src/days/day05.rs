use std::collections::HashSet;

use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day05.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let mut precedence = HashSet::new();

    input
        .lines()
        .take_while(|line| line.contains("|"))
        .map(|x| {
            x.split("|")
                .take(2)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|pair| {
            precedence.insert(pair[0] * 1000 + pair[1]);
        });

    let result = input
        .lines()
        .skip_while(|line| line.contains("|"))
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vec| check_vec(vec, &precedence))
        .map(|vec| vec[vec.len() / 2])
        .sum::<i32>();

    println!("{}", result);
}

fn check_vec(vec: &[i32], precedence: &HashSet<i32>) -> bool {
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if precedence.get(&(vec[j] * 1000 + vec[i])).is_some() {
                return false;
            }
        }
    }

    true
}

fn part2(input: &str) {
    let mut precedence = HashSet::new();

    input
        .lines()
        .take_while(|line| line.contains("|"))
        .map(|x| {
            x.split("|")
                .take(2)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|pair| {
            precedence.insert(pair[0] * 1000 + pair[1]);
        });

    let result = input
        .lines()
        .skip_while(|line| line.contains("|"))
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vec| !check_vec(vec, &precedence))
        .map(|vec| sort_vec(&vec, &precedence))
        .map(|vec| vec[vec.len() / 2])
        .sum::<i32>();

    println!("{}", result);
}

fn sort_vec(vec: &[i32], precedence: &HashSet<i32>) -> Vec<i32> {
    let mut vec_mut = vec.to_vec().clone();

    vec_mut.sort_by(|a, b| {
        if precedence.get(&(a * 1000 + b)).is_some() {
            std::cmp::Ordering::Less
        } else if precedence.get(&(b * 1000 + a)).is_some() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    vec_mut
}
