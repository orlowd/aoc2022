use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Range {
    start: u32,
    end: u32,
}

fn parse_section_range(rng: &str) -> Range {
    let sep = rng.find('-').unwrap();

    Range {
        start: rng[..sep].parse().unwrap(),
        end: rng[(sep + 1)..].parse().unwrap(),
    }
}

fn parse_pair_assignments(pair: &str) -> (Range, Range) {
    let sep = pair.find(',').unwrap();

    (parse_section_range(&pair[..sep]), parse_section_range(&pair[(sep + 1)..]))
}

fn range_contains_another(a: &Range, b: &Range) -> bool {
    let a_contains_b = (a.start <= b.start) && (a.end >= b.end);
    let b_contains_a = (a.start >= b.start) && (a.end <= b.end);

    a_contains_b || b_contains_a
}

fn ranges_overlap(a: &Range, b: &Range) -> bool {
    let (first, last) = if a.start <= b.start {
        (a, b)
    } else {
        (b, a)
    };

    range_contains_another(a, b)
    || ((first.end >= last.start) && (first.start <= last.end))
}

fn main() {
    let path = Path::new("inputs/day04.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut containments = 0;
    let mut overlaps = 0;

    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");

        let (left, right) = parse_pair_assignments(&line);

        containments += range_contains_another(&left, &right) as u32;
        overlaps += ranges_overlap(&left, &right) as u32;
    }

    println!("[Part 1] The amount of assignment pairs in which one range fully \
             contains the other is {}", containments);
    println!("[Part 2] The amount of assignment pairs that overlap is {}",
             overlaps);
}
