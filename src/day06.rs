use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn increment_or_insert<K: Eq + Hash>(map: &mut HashMap<K, u32>, key: K) {
    *map.entry(key).or_default() += 1;
}

fn decrement_or_delete<K: Eq + Hash>(map: &mut HashMap<K, u32>, key: K) {
    let value = match map.get_mut(&key) {
        None => return,
        Some(value) => value,
    };

    if *value == 1 {
        map.remove(&key);
    } else {
        *value -= 1;
    }
}

fn no_duplicates<K: Eq + Hash>(map: &HashMap<K, u32>) -> bool {
    map.values().all(|&v| v == 1)
}

fn find_start_marker<const WIN_SIZE: usize>(line: &[u8]) -> usize {
    let mut win = HashMap::<u8, u32>::new();
    for c in &line[..WIN_SIZE] {
        increment_or_insert(&mut win, *c);
    }

    if no_duplicates(&win) {
        return WIN_SIZE;
    }

    for (i, symbols) in line.windows(WIN_SIZE + 1).enumerate() {
        increment_or_insert(&mut win, symbols[WIN_SIZE]);
        decrement_or_delete(&mut win, symbols[0]);

        if no_duplicates(&win) {
            return WIN_SIZE + i + 1;
        }
    }

    panic!("could not find start marker!");
}

fn main() {
    let path = Path::new("inputs/day06.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut line = String::new();
    _ = reader.read_line(&mut line);
    let line = line.as_bytes();

    let sop_pos = find_start_marker::<4>(&line);
    println!("[Part 1] The amount of characters that have to be processed \
              before the first SOP marker is detected is {}", sop_pos);

    let som_pos = find_start_marker::<14>(&line);
    println!("[Part 2] The amount of characters that have to be processed \
              before the first SOM marker is detected is {}", som_pos);
}
