use crate::utils::input::read_file;

pub(crate) fn run(part: u32) {
    let input = read_file("inputs/day04.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mat = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 0),
        (-1, -1),
        (0, -1),
    ];
    let mut result = 0;

    for x in 0..grid.len() as isize {
        for y in 0..grid[0].len() as isize {
            for (dx, dy) in mat {
                let last_x = x + 3 * dx;
                let last_y = y + 3 * dy;

                if last_x < 0
                    || last_x >= grid.len() as isize
                    || last_y < 0
                    || last_y >= grid[0].len() as isize
                {
                    continue;
                }

                let xmas = (0..=3)
                    .map(|step| {
                        grid[x as usize + step * dx as usize][y as usize + step * dy as usize]
                    })
                    .collect::<String>();

                if xmas == "XMAS" {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}

fn rotate_3x3_matrix<T: Default + Sized + Copy>(kernel: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    let mut rotated = [[T::default(); 3]; 3];

    (0..3).for_each(|i| {
        (0..3).for_each(|j| {
            rotated[j][2 - i] = kernel[i][j];
        });
    });

    rotated
}

#[test]
fn test_rotate_3x3_matrix() {
    let kernel = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let rotated = rotate_3x3_matrix(&kernel);

    assert_eq!(rotated, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
}

fn part2(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pattern = [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']];
    let mut result = 0;

    for x in 0..grid.len() - 2 {
        for y in 0..grid[0].len() - 2 {
            let mut rotated_pattern = pattern;

            for _rotations in 0..=3 {
                let mut matches = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if rotated_pattern[i][j] != '.'
                            && rotated_pattern[i][j] != grid[x + i][y + j]
                        {
                            matches = false;
                            break;
                        }
                    }
                    if !matches {
                        break;
                    }
                }
                if matches {
                    result += 1;
                }
                rotated_pattern = rotate_3x3_matrix(&rotated_pattern);
            }
        }
    }

    println!("{}", result);
}
