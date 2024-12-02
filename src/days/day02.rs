use super::super::utils::input;

pub fn run(part: u32) {
    let input = input::read_file("inputs/day02.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn is_safe(reports: &[i32]) -> bool {
    let ordering = reports[0].cmp(&reports[1]);

    let ordered_same = (0..reports.len() - 1)
        .map(|x| reports[x].cmp(&reports[x + 1]))
        .all(|x| x == ordering);

    let close_together = reports
        .windows(2)
        .all(|x| i32::abs(x[0] - x[1]) >= 1 && i32::abs(x[0] - x[1]) <= 3);

    close_together && ordered_same
}

fn is_safe_extended(reports: &[i32]) -> bool {
    (0..reports.len()).any(|x| is_safe(&[&reports[..x], &reports[x + 1..]].concat()))
}

fn part1(input: &str) {
    let count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|reports| is_safe(&reports) as i32)
        .sum::<i32>();

    println!("{count}");
}

fn part2(input: &str) {
    let count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|reports| is_safe_extended(&reports) as i32)
        .sum::<i32>();

    println!("{count}");
}
