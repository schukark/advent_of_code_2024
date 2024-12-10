use std::collections::{HashMap, HashSet};

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day10.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn construct_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(vertex: (usize, usize), visited: &mut HashSet<(usize, usize)>, grid: &[Vec<usize>]) {
    visited.insert(vertex);

    for (dx, dy) in &DIRECTIONS {
        let new_vertex = ((vertex.0 as isize + dx), (vertex.1 as isize + dy));

        if new_vertex.0 < 0
            || new_vertex.1 < 0
            || new_vertex.0 >= grid.len() as isize
            || new_vertex.1 >= grid[0].len() as isize
        {
            continue;
        }

        let new_vertex = (new_vertex.0 as usize, new_vertex.1 as usize);

        if visited.contains(&new_vertex) {
            continue;
        }

        if grid[new_vertex.0][new_vertex.1] != grid[vertex.0][vertex.1] + 1 {
            continue;
        }

        visited.insert(new_vertex);
        dfs(new_vertex, visited, grid);
    }
}

fn part1(input: &str) {
    let grid = construct_grid(input);

    let mut starting_points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                starting_points.push((i, j));
            }
        }
    }

    let mut ending_points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 9 {
                ending_points.push((i, j));
            }
        }
    }

    let mut result_vec = Vec::new();

    for point in starting_points {
        let mut visited = HashSet::new();
        dfs(point, &mut visited, &grid);

        let mut count = 0;
        for end_point in &ending_points {
            if visited.contains(end_point) {
                count += 1;
            }
        }

        result_vec.push(count);
    }

    println!("{}", result_vec.iter().sum::<usize>());
}

fn dfs2(vertex: (usize, usize), visited: &mut HashMap<(usize, usize), usize>, grid: &[Vec<usize>]) {
    *visited.entry(vertex).or_default() += 1;

    for (dx, dy) in &DIRECTIONS {
        let new_vertex = ((vertex.0 as isize + dx), (vertex.1 as isize + dy));

        if new_vertex.0 < 0
            || new_vertex.1 < 0
            || new_vertex.0 >= grid.len() as isize
            || new_vertex.1 >= grid[0].len() as isize
        {
            continue;
        }

        let new_vertex = (new_vertex.0 as usize, new_vertex.1 as usize);

        if grid[new_vertex.0][new_vertex.1] != grid[vertex.0][vertex.1] + 1 {
            continue;
        }

        dfs2(new_vertex, visited, grid);
    }
}

fn part2(input: &str) {
    let grid = construct_grid(input);

    let mut starting_points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                starting_points.push((i, j));
            }
        }
    }

    let mut ending_points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 9 {
                ending_points.push((i, j));
            }
        }
    }

    let mut result_vec = Vec::new();

    for point in starting_points {
        let mut visited = HashMap::new();
        dfs2(point, &mut visited, &grid);

        let mut count = 0;
        for end_point in &ending_points {
            count += visited.get(end_point).copied().unwrap_or_default();
        }

        result_vec.push(count);
    }

    println!("{}", result_vec.iter().sum::<usize>());
}
