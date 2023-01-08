use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::{Add, Sub, Mul, Div};
use std::path::Path;

type OpFunc = fn(i64, i64) -> i64;

enum Job {
    Number(i64),
    Operation(String, String, OpFunc)
}

impl Job {
    fn parse(input: &str) -> (String, Self) {
        let mut iter = input.split_whitespace();

        let name = iter.next().unwrap().trim_end_matches(':').to_string();

        let left = iter.next().unwrap();
        let maybe_num = left.parse::<i64>();

        let job = if let Ok(val) = maybe_num {
            Job::Number(val)
        } else {
            let operand = iter.next().unwrap();
            let left = left.to_string();
            let right = iter.next().unwrap().to_string();

            let func = match operand {
                "+" => Add::add,
                "-" => Sub::sub,
                "*" => Mul::mul,
                "/" => Div::div,
                _ => panic!("unexpected operation found"),
            };

            Job::Operation(left, right, func)
        };

        (name, job)
    }
}

fn get_opposite_operand(op: OpFunc) -> OpFunc {
    if op == Add::add {
        Sub::sub
    } else if op == Sub::sub {
        Add::add
    } else if op == Mul::mul {
        Div::div
    } else if op == Div::div {
        Mul::mul
    } else {
        panic!("unexpected operation given");
    }
}

fn lookup(name: &String, cache: &mut HashMap<String, i64>, jobs: &HashMap<String, Job>) -> i64 {
    if let Some(&value) = cache.get(name) {
        return value;
    }

    let op = &jobs[name];

    match op {
        &Job::Number(value) => {
            cache.insert(name.to_owned(), value);

            value
        },
        &Job::Operation(ref left, ref right, operand) => {
            let left = lookup(left, cache, jobs);
            let right = lookup(right, cache, jobs);
            let result = operand(left, right);
            cache.insert(name.to_owned(), result);

            result
        },
    }
}

fn calculate(name: &String, jobs: &HashMap<String, Job>) -> i64 {
    let mut cache = HashMap::<String, i64>::new();

    lookup(name, &mut cache, jobs)
}

fn part1(jobs: &HashMap<String, Job>) {
    let result = calculate(&"root".to_string(), jobs);
    println!("[Part 1] The monkey named 'root' will yell a number {}", result);
}

fn get_left_expected(job: &Job, result: i64, jobs: &HashMap<String, Job>) -> i64 {
    let &Job::Operation(_, ref right, op) = job else {
        panic!("unexpected job type given");
    };

    let right = calculate(right, jobs);
    let op = get_opposite_operand(op);

    op(result, right)
}

fn get_right_expected(job: &Job, result: i64, jobs: &HashMap<String, Job>) -> i64 {
    let &Job::Operation(ref left, _, op) = job else {
        panic!("unexpected job type given");
    };

    let left = calculate(left, jobs);

    let (op, l, r) = if op == Add::add || op == Mul::mul {
        (get_opposite_operand(op), result, left)
    } else {
        (op, left, result)
    };

    op(l, r)
}

fn part2(jobs: &HashMap<String, Job>) {
    let mut queue = VecDeque::<(&String, i64)>::new();

    let &Job::Operation(ref left, ref right, _) = &jobs[&"root".to_string()] else {
        panic!("unexpected job was given for 'root'");
    };

    queue.push_back((right, calculate(left, jobs)));
    queue.push_back((left, calculate(right, jobs)));

    let mut result = 0;
    while let Some((name, expected)) = queue.pop_front() {
        if name == &"humn".to_string() {
            result = expected;
            break;
        }

        let job = &jobs[name];
        if let &Job::Operation(ref left, ref right, op) = job {
            queue.push_back((left, get_left_expected(job, expected, jobs)));

            if !(expected == 0 && op == Div::div) {
                queue.push_back((right, get_right_expected(job, expected, jobs)));
            }
        }
    }

    println!("[Part 2] The number to yell to pass 'root's equality test is {}", result);
}

fn main() {
    let path = Path::new("inputs/day21.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let jobs: HashMap<_, _> = reader
        .lines()
        .map(|l| Job::parse(&l.unwrap()))
        .collect();

    part1(&jobs);
    part2(&jobs);
}
