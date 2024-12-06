use std::collections::HashSet;

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day06.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn _draw_path(grid: &[Vec<char>], visited: &HashSet<(i32, i32)>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i as i32, j as i32)) {
                print!("X");
            } else {
                print!("{}", grid[i][j]);
            }
        }

        println!();
    }

    println!();
}

fn part1(input: &str) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut direction = (-1, 0);
    let mut coords = (0, 0);

    for i in 0..grid.len() {
        if coords != (0, 0) {
            break;
        }

        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                coords = (i as i32, j as i32);
                grid[i][j] = '.';
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(coords);

    loop {
        let new_coords = (coords.0 + direction.0, coords.1 + direction.1);

        if new_coords.0 >= grid.len() as i32
            || new_coords.0 < 0
            || new_coords.1 >= grid[0].len() as i32
            || new_coords.1 < 0
        {
            break;
        }

        if grid[new_coords.0 as usize][new_coords.1 as usize] == '#' {
            direction = (direction.1, -direction.0);
            continue;
        }

        visited.insert(new_coords);
        coords = new_coords;
    }

    println!("{}", visited.len());

    // draw_path(&grid, &visited);
}

fn part2(input: &str) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut coords = (0, 0);

    for i in 0..grid.len() {
        if coords != (0, 0) {
            break;
        }

        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                coords = (i as i32, j as i32);
                grid[i][j] = '.';
                break;
            }
        }
    }

    let mut loopy = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }

            grid[i][j] = '#';
            if find_loop(coords, &grid) {
                loopy += 1;
            }

            grid[i][j] = '.';
        }
    }

    println!("{}", loopy);
}

fn find_loop(mut coords: (i32, i32), grid: &[Vec<char>]) -> bool {
    let mut direction = (-1, 0);
    let mut visited = HashSet::new();
    visited.insert((coords, direction));

    loop {
        let new_coords = (coords.0 + direction.0, coords.1 + direction.1);

        if new_coords.0 >= grid.len() as i32
            || new_coords.0 < 0
            || new_coords.1 >= grid[0].len() as i32
            || new_coords.1 < 0
        {
            break;
        }

        if grid[new_coords.0 as usize][new_coords.1 as usize] == '#' {
            direction = (direction.1, -direction.0);
            continue;
        }

        if visited.contains(&(new_coords, direction)) {
            return true;
        }

        visited.insert((new_coords, direction));
        coords = new_coords;
    }

    false
}
