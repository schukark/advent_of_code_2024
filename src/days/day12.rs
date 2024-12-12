use std::collections::HashSet;

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day12.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

const MAT: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn dfs(
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    cur_area: &mut usize,
    cur_perimeter: &mut usize,
    current: (usize, usize),
) {
    // println!("Visiting {:#?}", current);
    visited.insert(current);
    *cur_area += 1;
    *cur_perimeter += 4;

    for (dx, dy) in MAT {
        let new_x = current.0 as isize + dx;
        let new_y = current.1 as isize + dy;

        if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize
        {
            continue;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        if grid[new_x][new_y] != grid[current.0][current.1] {
            continue;
        }

        *cur_perimeter -= 1;

        if visited.contains(&(new_x, new_y)) {
            continue;
        }

        dfs(grid, visited, cur_area, cur_perimeter, (new_x, new_y));
    }
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let mut cur_area = 0;
            let mut cur_perimeter = 0;

            dfs(
                &grid,
                &mut visited,
                &mut cur_area,
                &mut cur_perimeter,
                (i, j),
            );

            result += cur_area * cur_perimeter;
        }
    }

    println!("{}", result);
}

fn part2(input: &str) {}
