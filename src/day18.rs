use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn neighbours(&self) -> [Point; 6] {
        let &Point { x, y, z } = self;

        [
            Point { x: x - 1, y, z }, Point { x: x + 1, y, z },
            Point { x, y: y - 1, z }, Point { x, y: y + 1, z },
            Point { x, y, z: z - 1 }, Point { x, y, z: z + 1 },
        ]
    }
}

fn remove_same_elems<T>(vec: &mut Vec<T>)
where T: Eq + Hash + Default + Copy {
    let mut table = HashMap::<T, usize>::new();
    vec.iter().for_each(|&c| *table.entry(c).or_default() += 1);

    vec.retain(|e| table[e] == 1);
}

fn surface_area(points: &[Point]) -> usize {
    let mut sides: Vec<_> = points
        .iter()
        .map(|Point { x, y, z }| Point { x: x * 2, y: y * 2, z: z * 2 })
        .flat_map(|p| p.neighbours())
        .collect();

    remove_same_elems(&mut sides);

    sides.len()
}

fn inner_area(points: &[Point]) -> usize {
    let (min_x, max_x) = points
        .iter()
        .map(|&Point { x, .. }| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = points
        .iter()
        .map(|&Point { y, .. }| y)
        .minmax()
        .into_option()
        .unwrap();
    let (min_z, max_z) = points
        .iter()
        .map(|&Point { z, .. }| z)
        .minmax()
        .into_option()
        .unwrap();

    let mut seen = HashMap::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                let p = Point { x, y, z };
                if !points.contains(&p) {
                    seen.insert(p, false);
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Point { x: min_x - 1, y: min_y, z: min_z });

    while let Some(p) = queue.pop_front() {
        seen.insert(p, true);
        p.neighbours().iter().for_each(|n| {
            if !seen.get(n).unwrap_or(&true) && !queue.contains(n) {
                queue.push_back(*n);
            }
        })
    }

    let inner_points: Vec<_> = seen
        .into_iter()
        .filter(|&(_, was_seen)| !was_seen)
        .map(|(p, _)| p)
        .collect();
    surface_area(&inner_points)
}

fn main() {
    let path = Path::new("src/day18.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let droplets = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut iter = l.split(',');

            Point {
                x: iter.next().unwrap().parse().unwrap(),
                y: iter.next().unwrap().parse().unwrap(),
                z: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let surface = surface_area(&droplets);
    println!("[Part 1] The surface area of the scanned \
              lava droplet is {}", surface);

    let inner = inner_area(&droplets);
    println!("[Part 2] The outer surface area of the scanned \
              lava droplet is {}", surface - inner);
}
