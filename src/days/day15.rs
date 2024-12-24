use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day15.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let mut grid = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .fold(String::from(""), |acc, line| acc + line);

    let robot_position = grid
        .iter()
        .flatten()
        .enumerate()
        .find(|(_idx, symbol)| **symbol == '@')
        .map(|(idx, _s)| idx)
        .unwrap();

    let mut robot_position = (
        robot_position as isize / grid.len() as isize,
        robot_position as isize % grid.len() as isize,
    );

    for cur_move in moves.chars() {
        let direction = match cur_move {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid move"),
        };

        robot_position = make_move(&mut grid, robot_position, direction);
    }

    println!("{}", calc_coords(&grid));
}

// fn print_grid(grid: &[Vec<char>]) {
//     for line in grid.iter() {
//         println!("{}", String::from_iter(line));
//     }
// }

fn calc_coords(grid: &[Vec<char>]) -> usize {
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                result += 100 * i + j;
            }
        }
    }

    result
}

fn make_move(
    grid: &mut [Vec<char>],
    position: (isize, isize),
    direction: (isize, isize),
) -> (isize, isize) {
    let new_position = (position.0 + direction.0, position.1 + direction.1);
    let new_position = (new_position.0 as usize, new_position.1 as usize);

    if grid[new_position.0][new_position.1] == '#' {
        return position;
    }

    if grid[new_position.0][new_position.1] == '.' {
        grid[new_position.0][new_position.1] = '@';
        grid[position.0 as usize][position.1 as usize] = '.';
        return (new_position.0 as isize, new_position.1 as isize);
    }

    let mut count_boxes_path = 0;
    for i in 1.. {
        let new_position = (position.0 + direction.0 * i, position.1 + direction.1 * i);
        let new_position = (new_position.0 as usize, new_position.1 as usize);

        if grid[new_position.0][new_position.1] == 'O' {
            count_boxes_path += 1;
        } else if grid[new_position.0][new_position.1] == '#' {
            count_boxes_path = 0;
            break;
        } else {
            break;
        }
    }

    if count_boxes_path == 0 {
        return position;
    }

    count_boxes_path += 1;
    let new_position = (
        position.0 + direction.0 * count_boxes_path,
        position.1 + direction.1 * count_boxes_path,
    );
    let new_position = (new_position.0 as usize, new_position.1 as usize);

    grid[new_position.0][new_position.1] = 'O';
    grid[position.0 as usize][position.1 as usize] = '.';

    let new_position = (position.0 + direction.0, position.1 + direction.1);
    let new_position = (new_position.0 as usize, new_position.1 as usize);

    grid[new_position.0][new_position.1] = '@';

    (new_position.0 as isize, new_position.1 as isize)
}

fn part2(input: &str) {}
