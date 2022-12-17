use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn neighbors(heightmap: &Vec<Vec<u8>>, pos: Position) -> Vec<Position> {
    let bound = Position { x: heightmap[0].len() - 1, y: heightmap.len() - 1 };
    let cur_height = heightmap[pos.y][pos.x];

    let mut neighbors = Vec::new();

    if pos.x > 0 && heightmap[pos.y][pos.x - 1] + 1 >= cur_height {
        neighbors.push(Position { x: pos.x - 1, y: pos.y });
    }
    if pos.y > 0 && heightmap[pos.y - 1][pos.x] + 1 >= cur_height {
        neighbors.push(Position { x: pos.x, y: pos.y - 1 });
    }
    if pos.x < bound.x && heightmap[pos.y][pos.x + 1] + 1 >= cur_height {
        neighbors.push(Position { x: pos.x + 1, y: pos.y });
    }
    if pos.y < bound.y && heightmap[pos.y + 1][pos.x] + 1 >= cur_height {
        neighbors.push(Position { x: pos.x, y: pos.y + 1 });
    }

    neighbors
}

fn bfs(heightmap: &Vec<Vec<u8>>, end: Position) -> HashMap<Position, u32> {
    let mut steps = HashMap::new();
    steps.insert(end, 0);

    let mut frontier = VecDeque::new();
    frontier.push_back(end);

    while !frontier.is_empty() {
        let pos = frontier.pop_front().unwrap();
        for neighbor in neighbors(heightmap, pos) {
            if !steps.contains_key(&neighbor) {
                steps.insert(neighbor, steps[&pos] + 1);
                frontier.push_back(neighbor);
            }
        }
    }

    return steps;
}

fn main() {
    let path = Path::new("src/day12.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut heightmap: Vec<Vec<u8>> = Vec::new();
    heightmap.reserve(reader.by_ref().lines().count());

    _ = reader.rewind();

    let mut start_pos = Position { x: 0, y: 0 };
    let mut end_pos = Position { x: 0, y: 0 };

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("reading a line from an input file failed");

        heightmap.push(line.as_bytes().iter().enumerate().map(
            |(j, &height)| {
                (if height == b'S' {
                    start_pos = Position { x: j, y: i };

                    b'a'
                } else if height == b'E' {
                    end_pos = Position { x: j, y: i };

                    b'z'
                } else {
                    height
                }) - b'a'
            }
        ).collect());
    }

    let table = bfs(&heightmap, end_pos);

    let steps_from_start = table[&start_pos];
    println!("[Part 1] The fewest steps required to move from the starting location \
              to the location of the best signal is {}", steps_from_start);

    let min_steps = table.into_iter()
                         .filter(|(k, _)| heightmap[k.y][k.x] == 0)
                         .map(|(_, v)| v)
                         .min().unwrap();
    println!("[Part 2] The fewest steps required to move from any square of elevation 'a' \
              to the location of the best signal is {}", min_steps);
}
