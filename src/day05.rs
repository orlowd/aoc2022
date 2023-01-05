use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

fn print_top_boxes(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        let top = stack.last();
        if let Some(name) = top {
            print!("{}", *name as char);
        }
    }
    println!();
}

fn parse_move_data(input: &str) -> (u32, usize, usize) {
    let mut split_line = input.split_ascii_whitespace();

    let count = split_line.nth(1).unwrap().parse::<u32>().unwrap();
    let from = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let to = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;

    (count, from, to)
}

fn part1<R>(mut stacks: Vec<Vec<u8>>, reader: &mut BufReader<R>)
where R: std::io::Read {
    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");

        let (count, from, to) = parse_move_data(&line);

        for _ in 0..count {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        }
    }

    print!("[Part 1] After the rearrangement by the CrateMover 9000, \
            crates that end up on top are: ");
    print_top_boxes(&stacks);
}

fn part2<R>(mut stacks: Vec<Vec<u8>>, reader: &mut BufReader<R>)
where R: std::io::Read {
    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");

        let (count, from, to) = parse_move_data(&line);

        let start = stacks[from].len() - count as usize;
        let items: Vec<_> = stacks[from].drain(start..).collect();
        stacks[to].extend(items);
    }

    print!("[Part 2] After the rearrangement by the CrateMover 9001, \
            crates that end up on top are: ");
    print_top_boxes(&stacks);
}

fn main() {
    let path = Path::new("inputs/day05.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let count = (reader.by_ref().lines().next().unwrap().unwrap().len() + 1)
                / "[a] ".len();

    _ = reader.rewind();

    let mut init_positions: Vec<String> = reader.by_ref().lines()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| line.unwrap())
        .collect();
    init_positions.pop();

    let mut stacks = vec![Vec::<u8>::new(); count];

    for i in 0..count {
        for h in (0..init_positions.len()).rev() {
            let box_name = init_positions[h].as_bytes()[4 * i + 1];
            if box_name == b' ' {
                break;
            }
            stacks[i].push(box_name);
        }
    }

    let saved_pos = reader.seek(SeekFrom::Current(0)).unwrap();
    part1(stacks.clone(), reader.by_ref());

    _ = reader.seek(SeekFrom::Start(saved_pos));
    part2(stacks, reader.by_ref());
}
