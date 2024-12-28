use std::collections::BTreeSet;

use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day16.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const VALUES: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn get_dx_dy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

impl From<Direction> for usize {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .flatten()
        .enumerate()
        .find(|symbol| *symbol.1 == 'S')
        .map(|symbol| symbol.0)
        .unwrap();

    let finish = grid
        .iter()
        .flatten()
        .enumerate()
        .find(|symbol| *symbol.1 == 'E')
        .map(|symbol| symbol.0)
        .unwrap();

    let start = (start / grid.len(), start % grid.len());
    let finish = (finish / grid.len(), finish % grid.len());

    let mut queue = BTreeSet::new();
    let mut dist = vec![vec![vec![usize::MAX; 4]; grid[0].len()]; grid.len()];
    let mut mem = vec![vec![start; grid[0].len()]; grid.len()];

    queue.insert((0, start, Direction::Left));
    dist[start.0][start.1][Direction::Left as usize] = 0;

    while let Some((distance, current, direction)) = queue.pop_first() {
        if current == finish {
            println!("{}", distance);
            break;
        }

        let (dx, dy) = direction.get_dx_dy();
        let new_x = current.0 as isize + dx;
        let new_y = current.1 as isize + dy;

        if !(new_x < 0
            || new_x >= grid.len() as isize
            || new_y < 0
            || new_y >= grid[0].len() as isize)
        {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if grid[new_x][new_y] != '#'
                && dist[new_x][new_y][direction as usize]
                    > dist[current.0][current.1][direction as usize] + 1
            {
                queue.remove(&(
                    dist[new_x][new_y][direction as usize],
                    (new_x, new_y),
                    direction,
                ));
                dist[new_x][new_y][direction as usize] =
                    dist[current.0][current.1][direction as usize] + 1;
                mem[new_x][new_y] = current;

                queue.insert((
                    dist[new_x][new_y][direction as usize],
                    (new_x, new_y),
                    direction,
                ));
            }
        }

        for new_direction in Direction::VALUES {
            if direction == new_direction {
                continue;
            }

            if dist[current.0][current.1][new_direction as usize]
                > dist[current.0][current.1][direction as usize] + 1000
            {
                queue.remove(&(
                    dist[current.0][current.1][new_direction as usize],
                    (current.0, current.1),
                    new_direction,
                ));
                dist[current.0][current.1][new_direction as usize] =
                    dist[current.0][current.1][direction as usize] + 1000;

                queue.insert((
                    dist[current.0][current.1][new_direction as usize],
                    (current.0, current.1),
                    new_direction,
                ));
            }
        }
    }
}

fn part2(input: &str) {}
