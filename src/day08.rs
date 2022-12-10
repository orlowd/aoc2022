use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use ndarray::{Array2, Axis};

fn main() {
    let path = Path::new("src/day08.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = reader.by_ref().lines();
    let width = lines.next().unwrap().unwrap().len();
    let height = lines.count() + 1;

    _ = reader.rewind();

    let mut trees = Array2::<u8>::zeros((width, height));
    
    for (row, line) in reader.lines().enumerate() {
        let line = line.expect("reading a line from an input file failed");

        for (col, c) in line.as_bytes().iter().enumerate() {
            trees[[row, col]] = c - b'0';
        }
    }

    let mut visible = (trees.len_of(Axis(0)) - 1 + trees.len_of(Axis(1)) - 1) * 2;

    let mut is_visible = Array2::<bool>::default((width - 1, height - 1));

    for row in 1..(height - 1) {
        let mut max = trees[[row, 0]];
        for col in 1..(width - 1) {
            let cur = trees[[row, col]];
            if cur > max {
                max = cur;
                is_visible[[row, col]] = true;
            }
        }

        max = trees[[row, width - 1]];
        for col in (1..(width - 1)).rev() {
            let cur = trees[[row, col]];
            if cur > max {
                max = cur;
                is_visible[[row, col]] = true;
            }
        }
    }

    for col in 1..(width - 1) {
        let mut max = trees[[0, col]];
        for row in 1..(height - 1) {
            let cur = trees[[row, col]];
            if cur > max {
                max = cur;
                is_visible[[row, col]] = true;
            }
        }

        max = trees[[height - 1, col]];
        for row in (1..(height - 1)).rev() {
            let cur = trees[[row, col]];
            if cur > max {
                max = cur;
                is_visible[[row, col]] = true;
            }
        }
    }

    visible += is_visible.iter().fold(0, |acc, x| acc + *x as usize);
    println!("[Part 1] The amount of trees that are visible from outside \
              the grid is {}", visible);

    let mut scores = Array2::<usize>::ones((width - 1, height - 1));

    for row in 1..(height - 1) {
        let mut positions = [0; 10];
        for col in 1..(width - 1) {
            let cur = trees[[row, col]] as usize;
            let dist = col - positions[cur..].iter().max().unwrap();

            scores[[row, col]] *= dist;
            positions[cur] = col;
        }

        positions = [width - 1; 10];
        for col in (1..(width - 1)).rev() {
            let cur = trees[[row, col]] as usize;
            let dist = positions[cur..].iter().min().unwrap() - col;

            scores[[row, col]] *= dist;
            positions[cur] = col;
        }
    }

    for col in 1..(width - 1) {
        let mut positions = [0; 10];
        for row in 1..(height - 1) {
            let cur = trees[[row, col]] as usize;
            let dist = row - positions[cur..].iter().max().unwrap();

            scores[[row, col]] *= dist;
            positions[cur] = row;
        }

        positions = [height - 1; 10];
        for row in (1..(height - 1)).rev() {
            let cur = trees[[row, col]] as usize;
            let dist = positions[cur..].iter().min().unwrap() - row;

            scores[[row, col]] *= dist;
            positions[cur] = row;
        }
    }

    let score = scores.iter().max().unwrap();
    println!("[Part 2] The highest scenic score is {}", score);
}
