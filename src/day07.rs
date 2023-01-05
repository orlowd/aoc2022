use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

fn main() {
    let path = Path::new("inputs/day07.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut cur_dir = Vec::<String>::new();
    let mut dirs = HashMap::<String, u32>::new();

    let mut input = reader.lines().peekable();

    while let Some(line) = input.next() {
        let line = line.unwrap();

        let mut iter = line.split_whitespace();
        iter.next(); // skip '$'

        match iter.next().unwrap() {
            "cd" => {
                match iter.next().unwrap() {
                    ".." => _ = cur_dir.pop(),
                    "/" => cur_dir.clear(),
                    name => cur_dir.push(name.to_string()),
                };
            },
            "ls" => {
                loop {
                    let line = input.peek();
                    if let None = line {
                        break;
                    }

                    let line = line.unwrap().as_ref().unwrap();
                    if line.as_bytes()[0] == b'$' {
                        break;
                    }

                    let split = line.split_whitespace();
                    for (what, _) in split.tuples::<(_, _)>() {
                        if what == "dir" {
                            continue;
                        }

                        let file_sz = what.parse::<u32>().unwrap();

                        for i in 0..=cur_dir.len() {
                            dirs.entry("/".to_string() + &cur_dir[0..i].join("/"))
                                .and_modify(|size| *size += file_sz)
                                .or_insert(file_sz);
                        }
                    }

                    input.next();
                }
            },
            unknown => panic!("got unexpected command: {}", unknown),
        };
    }

    const MAX_SIZE: u32 = 100_000;
    let size_sum: u32 = dirs.values().filter(|sz| **sz <= MAX_SIZE).sum();
    println!("[Part 1] The sum of the total sizes of directories with size \
              of at most 100000 is {}", size_sum);

    const TOTAL_SIZE: u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;
    let used = dirs.get("/").unwrap();
    let free = TOTAL_SIZE - used;
    let to_free = UPDATE_SIZE - free;

    let mut sizes: Vec<_> = dirs.into_values().collect();
    sizes.sort_unstable();

    let del_size = sizes.into_iter().find(|sz| *sz >= to_free).unwrap();
    println!("[Part 2] The total size of the directory which deletion would \
              free up enough space is {}", del_size);
}
