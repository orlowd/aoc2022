use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

type Position = (i32, i32);

fn move_rope<const NUM_KNOTS: usize, I>(motions: I) -> usize
where I: Iterator<Item = std::io::Result<String>> {
    let mut knots = [(0, 0); NUM_KNOTS];

    let mut visited = HashSet::<Position>::new();
    visited.insert(*knots.last().unwrap());

    for motion in motions {
        let motion = motion.unwrap();

        let mut iter = motion.split_ascii_whitespace();

        let dir = iter.next().unwrap();
        let count = iter.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..count {
            let mov = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "R" => (1, 0),
                "L" => (-1, 0),
                _ => panic!("got unexpected direction!"),
            };

            let head = knots.first_mut().unwrap();
            *head = (head.0 + mov.0, head.1 + mov.1);

            for i in 0..(knots.len() - 1) {
                let head = knots[i];
                let mut tail = &mut knots[i + 1];

                let diff = ((head.0 - tail.0), (head.1 - tail.1));

                if (-1..=1).contains(&diff.0) && (-1..=1).contains(&diff.1) {
                    break;
                }

                if diff.0 != 0 {
                    tail.0 += diff.0.clamp(-1, 1);
                    tail.1 += diff.1.clamp(-1, 1);
                } else {
                    assert_eq!(diff.1.abs(), 2);
                    tail.1 += diff.1.clamp(-1, 1);
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    let path = Path::new("inputs/day09.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let two_knots_cnt = move_rope::<2, _>(reader.by_ref().lines());
    println!("[Part 1] The number of positions that the tail of the rope \
              visits at least once for the amount of knots of 2 is {}", two_knots_cnt);

    _ = reader.rewind();

    let ten_knots_cnt = move_rope::<10, _>(reader.lines());
    println!("[Part 2] The number of positions that the tail of the rope \
              visits at least once for the amount of knots of 10 is {}", ten_knots_cnt);
}
