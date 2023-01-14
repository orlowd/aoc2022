use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(byte: u8) -> Self {
        use Direction::*;
        match byte {
            b'^' => Up,
            b'v' => Down,
            b'<' => Left,
            b'>' => Right,
            x => panic!("invalid direction given: '{}'", x as char),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Blizzard {
    start: Position,
    dir: Direction,
}

type Map = Vec<Vec<bool>>;

fn precalculate_maps(blizzards: &Vec<Blizzard>, width: usize, height: usize) -> Vec<Map> {
    let count = lcm(width, height);

    (1..=count).map(|i| {
        let mut next_map = vec![vec![false; width]; height];

        for row in next_map.iter_mut() {
            row[0] = true;
            row[width - 1] = true;
        }
        for x in 1..(width - 1) {
            next_map[0][x] = true;
            next_map[height - 1][(width - 1) - x] = true;
        }

        for b in blizzards {
            let pos = match b.dir {
                Direction::Left | Direction::Right => {
                    let size = (width - 2) as isize;
                    let step = if b.dir == Direction::Left { -1 } else { 1 }
                        * i as isize;
                    let x = (((b.start.x - 1) as isize + step).rem_euclid(size) + 1)
                        as usize;

                    Position { x, y: b.start.y }
                },
                Direction::Up | Direction::Down => {
                    let size = (height - 2) as isize;
                    let step = if b.dir == Direction::Up { -1 } else { 1 }
                        * i as isize;
                    let y = (((b.start.y - 1) as isize + step).rem_euclid(size) + 1)
                        as usize;

                    Position { x: b.start.x, y }
                }
            };

            next_map[pos.y][pos.x] = true;
        }

        next_map
    }).collect()
}

fn _draw_map(map: &Map) {
    for row in map {
        for state in row {
            let sym = match state {
                false => '.',
                true => '#',
            };
            print!("{sym}");
        }
        println!();
    }
}

fn min_time_to_traverse(
    from: Position, to: Position, at_minute: usize,
    maps: &[Map], width: usize, height: usize
) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(from, at_minute)]);

    let rep_count = lcm(width, height);

    let mut steps = usize::MAX;

    const MOVEMENTS: [(isize, isize); 4] = [
        (0, 1),        // Down
        (1, 0),        // Right
        (-1, 0),       // Left
        (0, -1),       // Up
    ];

    while let Some((pos, iter)) = queue.pop_front() {
        if !visited.insert((pos, iter % rep_count)) {
            continue;
        }

        let next = iter + 1;
        if next >= steps {
            continue;
        }

        if pos == to {
            steps = steps.min(next);
            continue;
        }

        let map = &maps[iter];

        if !map[pos.y][pos.x] || pos.y == from.y {
            queue.push_back((pos, next));
        }

        if pos.y == from.y {
            let next_y = if from.y == 0 {
                from.y + 1
            } else {
                from.y - 1
            };

            if !map[next_y][pos.x] {
                queue.push_back((Position { x: pos.x, y: next_y }, next));
            }
            continue;
        }

        for (x, y) in MOVEMENTS {
            let next_pos = Position {
                x: (pos.x as isize + x) as usize,
                y: (pos.y as isize + y) as usize
            };

            if map[next_pos.y][next_pos.x] {
                continue;
            }

            queue.push_back((
                next_pos,
                next
            ));
        }
    }

    steps
}

fn main() {
    let path = Path::new("inputs/day24.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = reader.by_ref().lines();
    let width = lines.next().unwrap().unwrap().len();
    let height = lines.count() + 1;
    _ = reader.rewind();

    let blizzards: Vec<_> = reader
        .lines()
        .into_iter()
        .map(|l| l
            .unwrap()
            .bytes()
            .collect::<Vec<u8>>()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| !(*b == b'.' || *b == b'#')))
        .enumerate()
        .flat_map(|(y, inner)| inner.map(move |(x, b)| (y, (x, b))))
        .map(|(y, (x, dir))| Blizzard { 
            start: Position { x, y },
            dir: Direction::parse(dir) })
        .collect();

    let maps = precalculate_maps(&blizzards, width, height);

    let start_pos = Position { x: 1, y: 0 };
    let end_pos = Position { x: width - 2, y: height - 2 };
    let steps = min_time_to_traverse(start_pos, end_pos, 0, &maps, width, height);

    println!("[Part 1] The fewest number of minutes required to avoid the blizzards \
              and reach the goal is {steps}");

    let back_start_pos = Position { x: width - 2, y: height - 1 };
    let back_end_pos = Position { x: 1, y: 1 };
    let steps = min_time_to_traverse(back_start_pos, back_end_pos, steps, &maps, width, height);

    let steps = min_time_to_traverse(start_pos, end_pos, steps, &maps, width, height);

    println!("[Part 2] The fewest number of minutes required to reach the goal, \
              go back to the start, then reach the goal agan is {steps}");
}
