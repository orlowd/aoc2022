use core::cmp::{min, max};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

const SPAWN_X: usize = 500;
const SPAWN_Y: usize = 0;

type MinMax = (usize, usize);

fn init_map(
    (min_x, max_x): MinMax,
    (min_y, max_y): MinMax,
    paths: &Vec<Vec<Position>>
) -> Vec<Vec<bool>> {
    let mut map = vec![vec![false; max_x - min_x + 1]; max_y - min_y + 1];

    for path in paths {
        for win in path.windows(2) {
            let from = Position {
                x: win[0].x - min_x,
                y: win[0].y - min_y,
            };
            let to = Position {
                x: win[1].x - min_x,
                y: win[1].y - min_y,
            };

            if from.x == to.x {
                let start = min(from.y, to.y);
                let end = max(from.y, to.y);
                for y in start..=end {
                    map[y][to.x] = true;
                }
            } else if from.y == to.y {
                let start = min(from.x, to.x);
                let end = max(from.x, to.x);
                for x in start..=end {
                    map[to.y][x] = true;
                }
            }
        }
    }

    map
}

fn part1(
    (min_x, max_x): MinMax,
    (min_y, max_y): MinMax,
    paths: Vec<Vec<Position>>
) {
    let mut map = init_map((min_x, max_x), (min_y, max_y), &paths);

    let spawn_pos = Position {
        x: SPAWN_X - min_x,
        y: SPAWN_Y - min_y,
    };

    let mut count: u32 = 0;

    'spawn: loop {
        let mut next = spawn_pos;

        loop {
            if next.y + 1 == map.len() {
                break 'spawn;
            } else if map[next.y + 1][next.x] == false {
                next.y += 1;
            } else if next.x == 0 {
                break 'spawn;
            } else if map[next.y + 1][next.x - 1] == false {
                next.y += 1;
                next.x -= 1;
            } else if next.x + 1 == map[0].len() {
                break 'spawn;
            } else if map[next.y + 1][next.x + 1] == false {
                next.y += 1;
                next.x += 1;
            } else {
                map[next.y][next.x] = true;
                break;
            }
        }

        count += 1;
    }

    println!("[Part 1] The amount of units of sand that come to rest \
              before sand starts flowing into the abyss below is {}", count);
}

fn part2(
    (min_x, max_x): MinMax,
    (min_y, max_y): MinMax,
    paths: Vec<Vec<Position>>
) {
    let max_y = max_y + 2;
    let half_width = max_y - min_y;
    let max_x = max(max_x, SPAWN_X + half_width);
    let min_x = min(min_x, SPAWN_X - half_width);

    let mut map = init_map((min_x, max_x), (min_y, max_y), &paths);
    for x in map.last_mut().unwrap() {
        *x = true;
    }

    let spawn_pos = Position {
        x: SPAWN_X - min_x,
        y: SPAWN_Y - min_y,
    };

    let mut count: u32 = 0;

    loop {
        let mut next = spawn_pos;
        if map[next.y][next.x] == true {
            break;
        }

        loop {
            if map[next.y + 1][next.x] == false {
                next.y += 1;
            } else if next.x == 0 {
                panic!("next.x - 1 < 0");
            } else if map[next.y + 1][next.x - 1] == false {
                next.y += 1;
                next.x -= 1;
            } else if next.x + 1 == map[0].len() {
                panic!("next.x + 1 > map[0].len() - 1");
            } else if map[next.y + 1][next.x + 1] == false {
                next.y += 1;
                next.x += 1;
            } else {
                map[next.y][next.x] = true;
                break;
            }
        }

        count += 1;
    }

    println!("[Part 2] The amount of units of sand that come to rest \
              before the source is blocked is {}", count);
}

fn main() {
    let path = Path::new("inputs/day14.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let paths = reader
        .lines()
        .map(|l| l.unwrap().split(" -> ")
            .map(|s| {
                let (x, y) = s.split_once(',').unwrap();

                Position {
                    x: x.parse::<usize>().unwrap(),
                    y: y.parse::<usize>().unwrap()
                }
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ((min_x, max_x), (min_y, max_y)) = paths
        .iter()
        .flatten()
        .chain(iter::once(
            &Position {
                x: SPAWN_X, y: SPAWN_Y
            }))
        .fold(((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)), |acc, p| {
            (
                (min(acc.0.0, p.x), max(acc.0.1, p.x)),
                (min(acc.1.0, p.y), max(acc.1.1, p.y))
            )
    });

    part1((min_x, max_x), (min_y, max_y), paths.clone());
    part2((min_x, max_x), (min_y, max_y), paths);
}
