use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day07.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn solve1(numbers: &[i64], last_index: Option<usize>, cur_sum: i64, target: i64) -> bool {
    if cur_sum > target {
        return false;
    }

    if last_index.is_none() {
        return solve1(numbers, Some(0), numbers[0], target);
    }

    let last_index = last_index.unwrap();

    if last_index == numbers.len() - 1 {
        return cur_sum == target;
    }

    let add_result = solve1(
        numbers,
        Some(last_index + 1),
        cur_sum + numbers[last_index + 1],
        target,
    );
    let mul_result = solve1(
        numbers,
        Some(last_index + 1),
        cur_sum * numbers[last_index + 1],
        target,
    );

    add_result || mul_result
}

fn part1(input: &str) {
    let result = input
        .lines()
        .map(|line| line.split(":").collect::<Vec<_>>())
        .map(|line_vec| {
            (
                line_vec[0].parse::<i64>().unwrap(),
                line_vec[1]
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(target, numbers)| solve1(numbers, None, 0, *target))
        .map(|(value, _numbers)| value)
        .sum::<i64>();

    println!("{}", result);
}

fn concat(left: i64, right: i64) -> i64 {
    let mut left_str = left.to_string();
    left_str.push_str(&right.to_string());
    left_str.parse::<i64>().unwrap()
}

fn solve2(numbers: &[i64], last_index: Option<usize>, cur_sum: i64, target: i64) -> bool {
    if cur_sum > target {
        return false;
    }

    if last_index.is_none() {
        return solve2(numbers, Some(0), numbers[0], target);
    }

    let last_index = last_index.unwrap();

    if last_index == numbers.len() - 1 {
        return cur_sum == target;
    }

    let add_result = solve2(
        numbers,
        Some(last_index + 1),
        cur_sum + numbers[last_index + 1],
        target,
    );
    let mul_result = solve2(
        numbers,
        Some(last_index + 1),
        cur_sum * numbers[last_index + 1],
        target,
    );
    let concat_result = solve2(
        numbers,
        Some(last_index + 1),
        concat(cur_sum, numbers[last_index + 1]),
        target,
    );

    add_result || mul_result || concat_result
}

fn part2(input: &str) {
    let result = input
        .lines()
        .map(|line| line.split(":").collect::<Vec<_>>())
        .map(|line_vec| {
            (
                line_vec[0].parse::<i64>().unwrap(),
                line_vec[1]
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(target, numbers)| solve2(numbers, None, 0, *target))
        .map(|(value, _numbers)| value)
        .sum::<i64>();

    println!("{}", result);
}
