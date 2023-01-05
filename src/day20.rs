use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn mix_numbers(numbers: &Vec<i64>, times: u8) -> Vec<usize> {
    let mut indexes: Vec<usize> = (0..numbers.len()).collect();

    for _ in 0..times {
        for (i, &num) in numbers.iter().enumerate() {
            let old_pos = indexes.iter()
                .position(|&index| index == i).unwrap();

            indexes.remove(old_pos);

            let new_pos = (old_pos as i64 + num as i64)
                .rem_euclid(indexes.len() as i64) as usize;
            indexes.insert(new_pos, i);
        }
    }

    indexes
}

fn grove_coords(numbers: &Vec<i64>, indexes: &Vec<usize>) -> i64 {
    let zero_pos = numbers.iter().position(|&i| i == 0).unwrap();
    let zero_idx = indexes.iter().position(|&i| i == zero_pos).unwrap();

    [1000, 2000, 3000].into_iter().map(|idx| {
        let idx = (idx + zero_idx) % numbers.len();

        numbers[indexes[idx]]
    }).sum()
}

fn main() {
    let path = Path::new("inputs/day20.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut numbers: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let indexes = mix_numbers(&numbers, 1);
    let sum = grove_coords(&numbers, &indexes);

    println!("[Part 1] The sum of the three numbers that form the \
              grove coordinates is {}", sum);

    const DECRYPTION_KEY: i64 = 811589153;
    const ROUNDS: u8 = 10;

    numbers.iter_mut().for_each(|num| *num *= DECRYPTION_KEY );

    let indexes = mix_numbers(&numbers, ROUNDS);
    let sum = grove_coords(&numbers, &indexes);

    println!("[Part 2] The sum of the three numbers that form the \
              grove coordinates is actually {}", sum);
}
