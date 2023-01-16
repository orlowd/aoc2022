use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn parse_snafu_char(symbol: u8) -> i8 {
    match symbol {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("unexpected symbol given"),
    }
}

fn from_snafu(input: &str) -> u64 {
    input
        .bytes()
        .rev()
        .enumerate()
        .fold(0_i64, |acc, (i, b)|
            acc + parse_snafu_char(b) as i64 * 5_i64.pow(i as u32)
        ) as u64
}

fn to_snafu(mut val: u64) -> String {
    if val == 0 {
        return "0".to_string()
    }

    let mut s = vec![];

    while val != 0 {
        let digit = val % 5;

        s.push(match digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'=',
            4 => b'-',
            _ => panic!("wrong digit given")
        });

        val = (val + 2) / 5;
    }

    String::from_utf8(
        s.into_iter().rev().collect::<Vec<_>>()
    ).unwrap()
}

fn main() {
    let path = Path::new("inputs/day25.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let sum = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| from_snafu(&l))
        .sum();

    let answer = to_snafu(sum);
    println!("The SNAFU number to supply to Bob's console is {}", answer);
}
