use std::{fs::File, io::{BufRead, BufReader}, path::Path};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn main() {
    let path = Path::new("src/day02.txt");
    let reader = match File::open(&path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut score_first = 0;
    let mut score_second = 0;

    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");

        use crate::Shape::*;
        use crate::Result::*;

        let opponent = match line.as_bytes()[0] {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => panic!("unexpected opponent shape input"),
        };

        let (you, target) = match line.as_bytes()[2] {
            b'X' => (Rock, Lose),
            b'Y' => (Paper, Draw),
            b'Z' => (Scissors, Win),
            _ => panic!("unexpected your shape input"),
        };

        score_first += match you {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        score_second += match target {
            Lose => 0,
            Draw => 3,
            Win => 6,
        };

        let result = match you {
            Rock => {
                if let Paper = opponent {
                    Lose
                } else if let Scissors = opponent {
                    Win
                } else {
                    Draw
                }
            },
            Paper => {
                if let Scissors = opponent {
                    Lose
                } else if let Rock = opponent {
                    Win
                } else {
                    Draw
                }
            },
            Scissors => {
                if let Rock = opponent {
                    Lose
                } else if let Paper = opponent {
                    Win
                } else {
                    Draw
                }
            },
        };

        score_first += match result {
            Lose => 0,
            Draw => 3,
            Win => 6,
        };

        let target_you = match target {
            Lose => {
                if let Rock = opponent {
                    Scissors
                } else if let Paper = opponent {
                    Rock
                } else {
                    Paper
                }
            },
            Draw => {
                opponent
            },
            Win => {
                if let Rock = opponent {
                    Paper
                } else if let Paper = opponent {
                    Scissors
                } else {
                    Rock
                }
            }
        };

        score_second += match target_you {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
    }

    println!("Total score in first part is {}", score_first);
    println!("Total score in second part is {}", score_second);
}
