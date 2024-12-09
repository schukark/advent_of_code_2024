use std::fmt::Display;

use crate::utils::input;

pub(crate) fn run(part: u32) {
    let input = input::read_file("inputs/day09.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

type Block = Option<usize>;
struct FileSystem {
    files: Vec<usize>,
    blocks: Vec<Block>,
}

impl FileSystem {
    fn new(files: Vec<usize>) -> Self {
        FileSystem {
            files,
            blocks: Vec::new(),
        }
    }

    fn move_last_block(&mut self) -> Result<(), ()> {
        let last_block = self.blocks.last().copied().unwrap();

        let free_space = self.blocks.iter_mut().find(|x| x.is_none()).ok_or(())?;
        *free_space = last_block;
        self.blocks.pop();
        self.trim();

        Ok(())
    }

    fn trim(&mut self) {
        while self.blocks.last().unwrap().is_none() {
            self.blocks.pop();
        }
    }

    fn compact(&mut self) {
        while self.move_last_block().is_ok() {}
    }

    fn add_block(&mut self, id: usize, len: usize) {
        self.blocks.append(&mut vec![Some(id); len]);
    }

    fn add_space(&mut self, len: usize) {
        self.blocks.append(&mut vec![None; len]);
    }

    fn compute_value(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, value)| idx * value.unwrap())
            .sum::<usize>()
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .blocks
            .iter()
            .map(|x| {
                if x.is_some() {
                    x.unwrap().to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("");

        write!(f, "{}", result)
    }
}

fn part1(input: &str) {
    let input = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let files = input.chunks(2).map(|chunk| chunk[0]).collect::<Vec<_>>();
    let mut file_system = FileSystem::new(files);

    input.chunks(2).enumerate().for_each(|(idx, chunk)| {
        file_system.add_block(idx, chunk[0]);
        if chunk.len() > 1 {
            file_system.add_space(chunk[1]);
        }
    });

    println!("{}", file_system);
    file_system.compact();
    println!("{}", file_system.compute_value());
}

fn part2(input: &str) {}
