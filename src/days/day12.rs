use std::collections::{HashMap, HashSet, VecDeque};

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
    visited: &mut HashSet<(isize, isize)>,
    cur_area: &mut isize,
    cur_perimeter: &mut isize,
    current: (isize, isize),
) {
    // println!("Visiting {:#?}", current);
    visited.insert(current);
    *cur_area += 1;
    *cur_perimeter += 4;

    for (dx, dy) in MAT {
        let new_x = current.0 + dx;
        let new_y = current.1 + dy;

        if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize
        {
            continue;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        if grid[new_x][new_y] != grid[current.0 as usize][current.1 as usize] {
            continue;
        }

        *cur_perimeter -= 1;

        if visited.contains(&(new_x as isize, new_y as isize)) {
            continue;
        }

        dfs(
            grid,
            visited,
            cur_area,
            cur_perimeter,
            (new_x as isize, new_y as isize),
        );
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
            if visited.contains(&(i as isize, j as isize)) {
                continue;
            }

            let mut cur_area = 0;
            let mut cur_perimeter = 0;

            dfs(
                &grid,
                &mut visited,
                &mut cur_area,
                &mut cur_perimeter,
                (i as isize, j as isize),
            );

            result += cur_area * cur_perimeter;
        }
    }

    println!("{}", result);
}

fn dfs2(
    grid: &[Vec<char>],
    visited: &mut HashSet<(isize, isize)>,
    new_visited: &mut HashSet<(isize, isize)>,
    cur_area: &mut isize,
    current: (isize, isize),
) {
    // println!("Visiting {:#?}", current);
    visited.insert(current);
    new_visited.insert(current);
    *cur_area += 1;

    for (dx, dy) in MAT {
        let new_x = current.0 + dx;
        let new_y = current.1 + dy;

        if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize
        {
            continue;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        if grid[new_x][new_y] != grid[current.0 as usize][current.1 as usize] {
            continue;
        }

        if visited.contains(&(new_x as isize, new_y as isize)) {
            continue;
        }

        dfs2(
            grid,
            visited,
            new_visited,
            cur_area,
            (new_x as isize, new_y as isize),
        );
    }
}

fn count_neighbors(grid: &[Vec<char>], current: (isize, isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::new();

    for (dx, dy) in MAT {
        let new_x = current.0 + dx;
        let var_name = current.1;
        let new_y = var_name + dy;

        if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize
        {
            result.push((dx, dy));
            continue;
        }

        if grid[current.0 as usize][current.1 as usize] != grid[new_x as usize][new_x as usize] {
            result.push((dx, dy));
        }
    }

    result
}

fn border_count(grid: &[Vec<char>], visited: &HashSet<(isize, isize)>) -> isize {
    let mut border_count = 0;

    let border = visited
        .iter()
        .flat_map(|&point| {
            count_neighbors(grid, point)
                .iter()
                .map(|(dx, dy)| (point.0 + dx, point.1 + dy, *dx, *dy))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    println!("{:#?}", border.clone());

    let near_map = visited
        .iter()
        .map(|&point| (point, count_neighbors(grid, point)))
        .collect::<HashMap<(isize, isize), _>>();

    let mut visited = HashSet::new();
    for border_element in border.iter() {
        let mut queue = VecDeque::new();

        if visited.contains(border_element) {
            continue;
        }
        border_count += 1;

        queue.push_back(*border_element);

        while let Some(top_elem) = queue.pop_back() {
            println!("{:#?}", top_elem);
            visited.insert(top_elem);
            let (x, y, dx_old, dy_old) = top_elem;

            for (dx, dy) in MAT {
                let new_x = x + dx;
                let new_y = y + dy;

                for direction in near_map.get(&(new_x, new_y)).unwrap_or(&Vec::new()) {
                    if *direction != (dx_old, dy_old) {
                        continue;
                    }

                    println!("{} {} {} {}", new_x, new_y, direction.0, direction.1);

                    if visited.contains(&(new_x, new_y, direction.0, direction.1)) {
                        continue;
                    }

                    if !border.contains(&(new_x, new_y, direction.0, direction.1)) {
                        continue;
                    }

                    queue.push_back((new_x, new_y, direction.0, direction.1));
                }
            }
        }
    }

    border_count
}

fn part2(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i as isize, j as isize)) {
                continue;
            }

            let mut cur_area = 0;
            let mut new_visited = HashSet::new();

            dfs2(
                &grid,
                &mut visited,
                &mut new_visited,
                &mut cur_area,
                (i as isize, j as isize),
            );

            let border_count = border_count(&grid, &new_visited);
            println!("{:#?}", border_count);
            // panic!();
        }
    }

    println!("{}", result);
}
