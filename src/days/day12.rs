use std::collections::HashMap;

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
    visited: &mut HashMap<(usize, usize), isize>,
    cur_area: &mut usize,
    cur_perimeter: &mut usize,
    current: (usize, usize),
    region: isize,
) {
    // println!("Visiting {:#?}", current);
    visited.insert(current, region);
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

        if visited.contains_key(&(new_x, new_y)) {
            continue;
        }

        dfs(
            grid,
            visited,
            cur_area,
            cur_perimeter,
            (new_x, new_y),
            region,
        );
    }
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashMap::new();
    let mut result = 0;
    let mut region = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains_key(&(i, j)) {
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
                region,
            );

            result += cur_area * cur_perimeter;
            region += 1;
        }
    }

    println!("{}", result);
}

fn part2(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashMap::new();
    let mut result = 0;
    let mut region = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains_key(&(i, j)) {
                continue;
            }

            let mut cur_area = 0;
            let mut _cur_perimeter = 0;

            dfs(
                &grid,
                &mut visited,
                &mut cur_area,
                &mut _cur_perimeter,
                (i, j),
                region,
            );

            let corner_count = count_corners_region(&grid, region, &visited);

            result += cur_area * corner_count;
            region += 1;
        }
    }

    println!("{}", result);
}

const CORNERS: [(isize, isize); 4] = [(-1, -1), (0, -1), (-1, 0), (0, 0)];
fn count_corners_region(
    grid: &[Vec<char>],
    region: isize,
    visited: &HashMap<(usize, usize), isize>,
) -> usize {
    let mut result = 0;

    for i in 0..grid.len() + 1 {
        for j in 0..grid[0].len() + 1 {
            let mut corners = Vec::new();
            let current = (i, j);

            for (dx, dy) in CORNERS {
                let new_x = current.0 as isize + dx;
                let new_y = current.1 as isize + dy;

                if new_x < 0
                    || new_x >= grid.len() as isize
                    || new_y < 0
                    || new_y >= grid[0].len() as isize
                {
                    corners.push(false);
                    continue;
                }

                let (new_x, new_y) = (new_x as usize, new_y as usize);

                if visited.contains_key(&(new_x, new_y))
                    && *visited.get(&(new_x, new_y)).unwrap() == region
                {
                    corners.push(true);
                } else {
                    corners.push(false);
                }
            }

            if corners.iter().filter(|x| **x).count() % 2 == 1 {
                result += 1;
            } else if (corners[1] & corners[2]) ^ (corners[0] & corners[3]) {
                result += 2;
            }
        }
    }

    result
}
