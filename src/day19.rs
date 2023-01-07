use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::{Add, Sub};
use std::path::Path;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Resources {
    fn has_enough(&self, needed: &Resources) -> bool {
        self.ore >= needed.ore 
        && self.clay >= needed.clay
        && self.obsidian >= needed.obsidian
        && self.geode >= needed.geode
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(Clone, Debug)]
struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn parse(input: &str) -> Blueprint {
        let mut iter = input.split_terminator('.');

        let ore = iter.next().unwrap().split_whitespace().nth(6).unwrap().parse().unwrap();
        let ore_robot = Resources { ore, ..Default::default() };

        let ore = iter.next().unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
        let clay_robot = Resources { ore, ..Default::default() };

        let mut obsidian_robot = iter.next().unwrap().split_whitespace();
        let ore = obsidian_robot.nth(4).unwrap().parse().unwrap();
        let clay = obsidian_robot.nth(2).unwrap().parse().unwrap();
        let obsidian_robot = Resources { ore, clay, ..Default::default() };

        let mut geode_robot = iter.next().unwrap().split_whitespace();
        let ore = geode_robot.nth(4).unwrap().parse().unwrap();
        let obsidian = geode_robot.nth(2).unwrap().parse().unwrap();
        let geode_robot = Resources { ore, obsidian, ..Default::default() };

        Blueprint { ore_robot, clay_robot, obsidian_robot, geode_robot }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Robots {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Entry {
    time_left: u8,
    resources: Resources,
    robots: Robots,
}

fn get_max_geode_count(blueprint: Blueprint, time_limit: u8) -> u8 {
    let mut stack = vec![Entry {
        time_left: time_limit,
        resources: Resources { ..Default::default() },
        robots: Robots { ore: 1, ..Default::default() }
    }];
    let mut seen = HashSet::new();
    let mut result = 0;

    let max_required_ore = [
        blueprint.ore_robot.ore,
        blueprint.clay_robot.ore,
        blueprint.obsidian_robot.ore,
        blueprint.geode_robot.ore
    ].into_iter().max().unwrap();

    let max_required_clay = [
        blueprint.ore_robot.clay,
        blueprint.clay_robot.clay,
        blueprint.obsidian_robot.clay,
        blueprint.geode_robot.clay
    ].into_iter().max().unwrap();

    let max_required_obsidian = [
        blueprint.ore_robot.obsidian,
        blueprint.clay_robot.obsidian,
        blueprint.obsidian_robot.obsidian,
        blueprint.geode_robot.obsidian
    ].into_iter().max().unwrap();

    let arith_prog_sum = |from, to| (to - from) * (from + to) / 2;

    while let Some(entry) = stack.pop() {
        if !seen.insert(entry.clone()) {
            continue;
        }

        let Entry { time_left, resources, robots } = entry;

        let time_left = time_left - 1;
        if time_left == 0 {
            result = result.max(resources.geode + robots.geode);
            continue;
        }

        let theor_max = resources.geode as u16 + arith_prog_sum(
            robots.geode as u16, (robots.geode + time_left + 1) as u16
        );
        if theor_max <= result as u16 {
            continue;
        }

        let collected = Resources {
            ore: robots.ore,
            clay: robots.clay,
            obsidian: robots.obsidian,
            geode: robots.geode,
        };

        if resources.has_enough(&blueprint.geode_robot) {
            stack.push(Entry {
                time_left,
                resources: resources - blueprint.geode_robot + collected,
                robots: Robots {
                    geode: robots.geode + 1,
                    ..robots.clone()
                }
            });
            continue;
        }

        if robots.ore < max_required_ore
           && resources.has_enough(&blueprint.ore_robot) {
            stack.push(Entry {
                time_left,
                resources: resources - blueprint.ore_robot + collected,
                robots: Robots {
                    ore: robots.ore + 1,
                    ..robots.clone()
                }
            });
        }
        if robots.clay < max_required_clay
           && resources.has_enough(&blueprint.clay_robot) {
            stack.push(Entry {
                time_left,
                resources: resources - blueprint.clay_robot + collected,
                robots: Robots {
                    clay: robots.clay + 1,
                    ..robots.clone()
                }
            });
        }
        if robots.obsidian < max_required_obsidian
           && resources.has_enough(&blueprint.obsidian_robot) {
            stack.push(Entry {
                time_left,
                resources: resources - blueprint.obsidian_robot + collected,
                robots: Robots {
                    obsidian: robots.obsidian + 1,
                    ..robots.clone()
                }
            });
        }

        if robots.ore < max_required_ore
           || robots.clay < max_required_clay
           || robots.obsidian < max_required_obsidian {
            stack.push(Entry {
                time_left,
                resources: resources + collected,
                robots: robots
            });
        }
    }

    result
}

fn part1<'a>(blueprints: impl Iterator<Item = &'a Blueprint>) {
    const TIME_LIMIT: u8 = 24;

    let result: usize = blueprints
        .enumerate()
        .map(|(i, bp)| (i + 1) * get_max_geode_count(bp.clone(), TIME_LIMIT) as usize)
        .sum();

    println!("[Part 1] The sum of quality levels of all of the blueprints is {}",
             result);
}

fn part2<'a>(blueprints: impl Iterator<Item = &'a Blueprint>) {
    const TIME_LIMIT: u8 = 32;

    let result: u32 = blueprints
        .take(3)
        .map(|bp| get_max_geode_count(bp.clone(), TIME_LIMIT) as u32)
        .product();

    println!("[Part 2] The multiple of the largest number of geodes that could be \
              opened using the first three blueprints is {}", result);
}

fn main() {
    let path = Path::new("inputs/day19.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let blueprints: Vec<_> = reader
        .lines()
        .map(|l| Blueprint::parse(&l.unwrap()))
        .collect();

    part1(blueprints.iter());
    part2(blueprints.iter());
}
