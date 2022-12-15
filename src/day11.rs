use std::fs::File;
use std::cmp::Reverse;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn parse_monkey_start_items(line: String) -> Vec<u64> {
    let mut split = line.split_ascii_whitespace();
    let _ = split.nth(1);

    let mut start_items = Vec::new();

    for item in split {
        let item_idx = item.trim_end_matches(',')
                           .parse::<u64>().unwrap();
        start_items.push(item_idx);
    }

    start_items
}

#[derive(Clone)]
enum Operand {
    Input,
    Const(u64),
}

#[derive(Clone)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    pub fn evaluate(&self, input: u64) -> u64 {
        use self::Operation::*;

        match self {
            Add(l, r) => Self::eval_operand(l, input)
                         + Self::eval_operand(r, input),
            Multiply(l, r) => Self::eval_operand(l, input)
                              * Self::eval_operand(r, input),
        }
    }

    fn eval_operand(operand: &Operand, input: u64) -> u64 {
        use self::Operand::*;

        match operand {
            Input => input,
            Const(val) => *val,
        }
    }
}

fn parse_operand(string: &str) -> Operand {
    use self::Operand::*;

    match string {
        "old" => Input,
        x => Const(x.parse::<u64>().unwrap()),
    }
}

fn parse_monkey_operation(line: String) -> Operation {
    let mut split = line.split_ascii_whitespace();

    let left = split.nth(3).unwrap();
    let op = split.next().unwrap();
    let right = split.next().unwrap();

    let l = parse_operand(left);
    let r = parse_operand(right);

    use self::Operation::*;

    match op {
        "+" => Add(l, r),
        "*" => Multiply(l, r),
        _ => panic!("unknown operation given: {}", op),
    }
}

#[derive(Clone)]
struct PassTo {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

impl PassTo {
    fn evaluate(&self, input: u64) -> usize {
        if input % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn parse_monkey_throw<I>(lines: &mut I) -> PassTo
where I: Iterator<Item = std::io::Result<String>> {
    let test_ln = lines.next().unwrap().unwrap();
    let divisible_by = test_ln.split_ascii_whitespace()
                              .last().unwrap()
                              .parse::<u64>().unwrap();

    let if_true_ln = lines.next().unwrap().unwrap();
    let if_true = if_true_ln.split_ascii_whitespace()
                            .last().unwrap()
                            .parse::<usize>().unwrap();

    let if_false_ln = lines.next().unwrap().unwrap();
    let if_false = if_false_ln.split_ascii_whitespace()
                              .last().unwrap()
                              .parse::<usize>().unwrap();

    PassTo { divisible_by, if_true, if_false }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    throws_to: PassTo,
}

fn gcd<T>(mut a: T, mut b: T) -> T
where T: Default + PartialOrd + std::ops::Rem<Output = T> + Clone {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b != T::default() {
        let t = b.clone();
        b = a % b;
        a = t;
    }

    a
}

fn lcm<T>(a: T, b: T) -> T
where T: Default + PartialOrd
         + Clone + Copy
         + std::ops::Mul<Output = T>
         + std::ops::Div<Output = T>
         + std::ops::Rem<Output = T> {
    a * b / gcd(a, b)
}

fn play_keep_away<F>(mut monkeys: Vec<Monkey>, num_rounds: u16,
                     manage_worry: F) -> u64
where F: FnOnce(u64) -> u64 + Copy {
    let mut items_inspected = vec![0; monkeys.len()];

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            let monkey_items: Vec<u64> = monkeys[i].items.drain(..).collect();
            for item in monkey_items {
                let worry = manage_worry(monkeys[i].operation.evaluate(item));
                let pass_to = monkeys[i].throws_to.evaluate(worry);

                items_inspected[i] += 1;

                monkeys[pass_to].items.push(worry);
            }
        }
    }

    items_inspected.sort_unstable_by_key(|x| Reverse(*x));

    items_inspected[..2].into_iter().product()
}

fn main() {
    let path = Path::new("src/day11.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = reader.lines().peekable();

    let mut monkeys = Vec::new();

    while let Some(_) = lines.peek() {
        let line = lines.nth(1).unwrap().unwrap();
        let items = parse_monkey_start_items(line);

        let line = lines.next().unwrap().unwrap();
        let operation = parse_monkey_operation(line);

        let throws_to = parse_monkey_throw(&mut lines);

        monkeys.push(Monkey { items, operation, throws_to });

        lines.next();
    }

    const NUM_ROUNDS_PT1: u16 = 20;
    let monkey_business = play_keep_away(monkeys.clone(), NUM_ROUNDS_PT1,
                                         |x| x / 3);
    println!("[Part 1] The level of monkey business after {} rounds of \
              stuff-slinging simian shenanigans is {}",
             NUM_ROUNDS_PT1, monkey_business);

    let pass_lcm = monkeys.iter().fold(
        1, |acc, monkey| lcm(acc, monkey.throws_to.divisible_by)
    );

    const NUM_ROUNDS_PT2: u16 = 10_000;
    let monkey_business = play_keep_away(monkeys.clone(), NUM_ROUNDS_PT2,
                                         |x| x % pass_lcm);
    println!("[Part 2] The level of monkey business after {} rounds of \
              stuff-slinging simian shenanigans is {}",
             NUM_ROUNDS_PT2, monkey_business);
}
