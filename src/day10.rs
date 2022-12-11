use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("src/day10.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut cycle_count: u32 = 0;
    let mut reg: i32 = 1;

    let mut signal_strengths_sum = 0;

    let to_check = [20, 60, 100, 140, 180, 220];
    let mut next_to_check = to_check.iter().peekable();

    let mut screen_buf = String::new();

    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");

        let mut iter = line.split_ascii_whitespace();

        let instr = iter.next().unwrap();
        let (value, cycles) = match instr {
            "noop" => {
                (0, 1)
            },
            "addx" => {
                let value = iter.next().unwrap().parse::<i32>().unwrap();

                (value, 2)
            },
            _ => panic!("got unknown instruction!"),
        };

        for i in 0..cycles {
            let pos = (cycle_count + i) % 40;

            if pos == 0 {
                screen_buf += "\n";
            }

            screen_buf += if ((reg - 1)..=(reg + 1)).contains(&(pos as i32)) {
                "#"
            } else {
                "."
            }
        }

        cycle_count += cycles;

        if let Some(cyc_to_check) = next_to_check.peek() {
            if cycle_count >= **cyc_to_check {
                let signal_strength = reg * (**cyc_to_check as i32);
                signal_strengths_sum += signal_strength;
                next_to_check.next();
            }
        }

        reg += value;
    }

    println!("[Part 1] The sum of the six signal strengths is {}", signal_strengths_sum);
    println!("[Part 2] The image given by the program: {}", screen_buf);
}
