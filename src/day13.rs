use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;

#[derive(Debug, Clone, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        order_pair(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Packet {
    fn parse_from(string: &str) -> Packet {
        let bytes = string.as_bytes();
        assert_eq!(bytes[0], b'[');

        let (list, _) = parse_list(&bytes[1..]);

        Packet::List(list)
    }
}

fn parse_list(bytes: &[u8]) -> (Vec<Packet>, usize) {
    let mut list = Vec::new();

    let mut i = 0;

    if bytes[i] == b']' {
        i += 1;
        return (list, i);
    }

    loop {
        if bytes[i] == b'[' {
            i += 1;

            let (inner_list, i_step) = parse_list(&bytes[i..]);

            i += i_step;
            list.push(Packet::List(inner_list))
        } else {
            let j = i + str::from_utf8(&bytes[i..]).unwrap()
                        .find(|x: char| !x.is_digit(10)).unwrap();

            let value = str::from_utf8(&bytes[i..j]).unwrap()
                        .parse::<u32>().unwrap();

            list.push(Packet::Int(value));
            i = j;
        }

        if bytes[i] == b',' {
            i += 1;
        } else if bytes[i] == b']' {
            i += 1;
            break;
        } else {
            panic!("unexpected byte @ {}: {}", i, bytes[i] as char);
        }
    }

    (list, i)
}

fn order_pair(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (&Packet::Int(l), &Packet::Int(r)) => {
            l.cmp(&r)
        },
        (&Packet::Int(l), r @ Packet::List(_)) => {
            order_pair(&Packet::List(vec![Packet::Int(l)]), r)
        },
        (l @ Packet::List(_), &Packet::Int(r)) => {
            order_pair(l, &Packet::List(vec![Packet::Int(r)]))
        },
        (&Packet::List(ref l), &Packet::List(ref r)) => {
            for (l, r) in l.iter().zip(r) {
                match order_pair(l, r) {
                    Ordering::Equal => continue,
                    ordering => return ordering,
                }
            }

            l.len().cmp(&r.len())
        }
    }
}

fn main() {
    let path = Path::new("src/day13.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = reader.lines().peekable();

    let mut i: usize = 1;
    let mut idx_sum = 0;

    let mut packets = Vec::new();

    while let Some(_) = lines.peek() {
        let left = Packet::parse_from(&lines.next().unwrap().unwrap());
        let right = Packet::parse_from(&lines.next().unwrap().unwrap());

        if left < right {
            idx_sum += i;
        }

        packets.push(left);
        packets.push(right);

        lines.next();
        i += 1;
    }

    println!("[Part 1] The sum of the indices of pairs of packets that are \
              in the right order is {}", idx_sum);

    let first_div = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let second_div = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.push(first_div.clone());
    packets.push(second_div.clone());

    packets.sort();

    let first_idx = packets.binary_search(&first_div).unwrap() + 1;
    let second_idx = packets.binary_search(&second_div).unwrap() + 1;

    let product = first_idx * second_idx;

    println!("[Part 2] The decoder key for the distress signal is {}",
             product);
}
