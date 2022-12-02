use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() {
    let path = Path::new("src/day01.txt");
    let reader = match File::open(&path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut calories: Vec<Vec<u32>> = Vec::new();
    calories.push(Vec::new());

    for line in reader.lines() {
        let line = line.expect("reading a line from an input file failed");
        if line.is_empty() {
            let cur_elf = Vec::new();
            calories.push(cur_elf);
        } else {
            let cur_elf = calories.last_mut().unwrap();
            let cur_amount: u32 = line.parse().expect("failed to parse input number");
            cur_elf.push(cur_amount);
        }
    }

    let mut elf_calories: Vec<u32> = calories.iter()
                                    .map(|elf: &Vec<u32>| elf.iter().sum::<u32>())
                                    .collect();

    let max_three = elf_calories.select_nth_unstable_by(3, |a, b| b.cmp(a)).0;

    let elf_max_calories = max_three.iter().max().unwrap();
    let top_three_sum: u32 = max_three.iter().sum();

    println!("Maximum amount of calories that one of elfs has is {}", elf_max_calories);
    println!("Sum of the top three calories amounts is {}", top_three_sum);
}
