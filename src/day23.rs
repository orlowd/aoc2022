use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

const MOVES: [(Position, Position, Position); 4] = [
    (Position { x: 0, y: -1 }, Position { x: -1, y: -1 }, Position { x: 1, y: -1 }),    // N, NE, NW
    (Position { x: 0, y: 1 }, Position { x: -1, y: 1 }, Position { x: 1, y: 1 }),       // S, SE, SW
    (Position { x: -1, y: 0 }, Position { x: -1, y: -1 }, Position { x: -1, y: 1 }),    // W, NW, SW
    (Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: -1 }),       // E, NE, SE
];

fn has_any_elf_nearby(elf: Position, positions: &HashSet<Position>) -> bool {
    const NEIGHBORS: [Position; 8] = [
        Position { x: 0, y: -1 }, Position { x: 1, y: -1 },
        Position { x: 1, y: 0 }, Position { x: 1, y: 1 },
        Position { x: 0, y: 1 }, Position { x: -1, y: 1 },
        Position { x: -1, y: 0 }, Position { x: -1, y: -1 }
    ];

    NEIGHBORS
        .iter()
        .map(|&Position { x, y }| Position { x: elf.x + x, y: elf.y + y })
        .map(|p| positions.contains(&p))
        .any(|x| x)
}

fn find_rect_extents(positions: &HashSet<Position>) -> ((isize, isize), (isize, isize)) {
    let (&min_x, &max_x) = positions
        .iter()
        .map(|Position { x, .. }| x)
        .minmax()
        .into_option()
        .unwrap();

    let (&min_y, &max_y) = positions
        .iter()
        .map(|Position { y, .. }| y)
        .minmax()
        .into_option()
        .unwrap();

    ((min_x, max_x), (min_y, max_y))
}

fn _draw_positions(positions: &HashSet<Position>) {
    let ((min_x, max_x), (min_y, max_y)) = find_rect_extents(positions);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            };
        }
        println!();
    }
    println!();
}

fn run_round(i: usize, positions: &mut HashSet<Position>) -> bool {
    let mut proposed = HashMap::<Position, (usize, Position)>::new();

    for &elf in positions.iter() {
        if !has_any_elf_nearby(elf, &positions) {
            continue;
        }

        let cyclic_iter = MOVES
            .iter()
            .cycle()
            .skip(i % MOVES.len())
            .take(MOVES.len());

        for mv in cyclic_iter {
            let can_move = [mv.0, mv.1, mv.2]
                .iter()
                .map(|p| positions.contains(&Position {x: elf.x + p.x, y: elf.y + p.y }))
                .all(|x| !x);

            if !can_move {
                continue;
            }

            let next_pos = Position { x: elf.x + mv.0.x, y: elf.y + mv.0.y };
            proposed.entry(next_pos).and_modify(|e| e.0 += 1).or_insert((1, elf));

            break;
        }
    }

    let mut movement_stopped = true;

    proposed
        .into_iter()
        .filter_map(|(end_pos, (count, start_pos))| {
            if count > 1 {
                None
            } else {
                Some((start_pos, end_pos))
            }
        })
        .for_each(|(start, end)| {
            movement_stopped = false;

            positions.remove(&start);
            positions.insert(end);
        });

    movement_stopped
}

fn main() {
    let path = Path::new("inputs/day23.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut positions: HashSet<_> = reader
        .lines()
        .into_iter()
        .map(|l| l
            .unwrap()
            .bytes()
            .collect::<Vec<u8>>()
            .into_iter()
            .enumerate())
        .enumerate()
        .flat_map(|(y, inner)| inner.map(move |(x, b)| (y, (x, b))))
        .filter(|(_, (_, b))| *b == b'#')
        .map(|(y, (x, _))| Position { x: x as isize, y: y as isize })
        .collect();

    const PART1_NUM_ROUNDS: usize = 10;

    for i in 0..PART1_NUM_ROUNDS {
        _ = run_round(i, &mut positions);
    }

    let num_elves = positions.len() as isize;

    let ((min_x, max_x), (min_y, max_y)) = find_rect_extents(&positions);

    let area = (max_x + 1 - min_x) * (max_y + 1 - min_y);
    let tiles_empty = area - num_elves;

    println!("[Part 1] The amount of empty ground tiles in a rectangle \
              is {}", tiles_empty);

    let mut i = PART1_NUM_ROUNDS;
    while run_round(i, &mut positions) == false {
        i += 1;
    }

    println!("[Part 2] The number of the first round in which no Elf \
              moves is {}", i + 1);
}
