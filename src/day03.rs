use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

use itertools::Itertools;

fn get_priority(item_type: u8) -> u8 {
    match item_type {
        b'a'..=b'z' => item_type - b'a' + 1,
        b'A'..=b'Z' => item_type - b'A' + 27,
        _ => panic!("unexpected item type given"),
    }
}

fn find_common_item_type_in_compartments(first: &[u8], second: &[u8]) -> u8 {
    let mut items_in_first = HashSet::new();

    for char in first {
        items_in_first.insert(char);
    }

    for char in second {
        if items_in_first.contains(char) {
            return *char;
        }
    }

    panic!("there was no common item in two compartments!");
}

fn find_common_item_type_in_group(
    first: &[u8], second: &[u8], third: &[u8]
) -> u8 {
    let mut items_in_first = HashSet::new();
    let mut common_in_two = HashSet::new();

    for char in first {
        items_in_first.insert(char);
    }

    for char in second {
        if items_in_first.contains(char) {
            common_in_two.insert(char);
        }
    }

    for char in third {
        if common_in_two.contains(char) {
            return *char;
        }
    }

    panic!("there was no common item in a group!");
}

fn main() {
    let path = Path::new("src/day03.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut compartments_priorities_sum = 0;

    for line in reader.by_ref().lines() {
        let line = line.expect("reading a line from an input file failed");

        let compartment_size = line.len() / 2;
        let common_item = find_common_item_type_in_compartments(
            &line.as_bytes()[..compartment_size],
            &line.as_bytes()[compartment_size..]
        );

        compartments_priorities_sum += get_priority(common_item) as u32;
    }

    // restart BufReader
    _ = reader.seek(SeekFrom::Start(0));

    let mut badges_priorities_sum = 0;

    for group in reader.lines().tuples::<(_, _, _)>() {
        let common_item = find_common_item_type_in_group(
            group.0.unwrap().as_bytes(),
            group.1.unwrap().as_bytes(),
            group.2.unwrap().as_bytes()
        );

        badges_priorities_sum += get_priority(common_item) as u32;
    }

    println!("[Part 1] The sum of the priorities of the item types that are common \
             in two compartments is {}", compartments_priorities_sum);
    println!("[Part 2] The sum of the priorities of the item types that correspond \
             to badges is {}", badges_priorities_sum);
}
