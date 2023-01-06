use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Debug, Clone, Copy)]
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

    fn subtract(&self, amount: &Resources) -> Resources {
        Resources {
            ore: self.ore - amount.ore,
            clay: self.clay - amount.clay,
            obsidian: self.obsidian - amount.obsidian,
            geode: self.geode - amount.geode,
        }
    }

    fn add(&self, amount: &Resources) -> Resources {
        Resources {
            ore: self.ore + amount.ore,
            clay: self.clay + amount.clay,
            obsidian: self.obsidian + amount.obsidian,
            geode: self.geode + amount.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

fn parse_blueprint(input: &str) -> Blueprint {
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

#[derive(Default, Clone, Debug)]
struct Robots {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

#[derive(Debug)]
struct Entry {
    time_left: u8,
    resources: Resources,
    robots: Robots,
}

fn get_max_geode_count(blueprint: Blueprint) -> u32 {
    let mut stack = vec![Entry {
        time_left: 24,
        resources: Resources { ..Default::default() },
        robots: Robots { ore: 1, ..Default::default() }
    }];
    let mut result = 0;

    while let Some(entry) = stack.pop() {
        let time_left = entry.time_left - 1;
        if time_left == 0 {
            result = result.max(entry.resources.geode + entry.robots.geode as u8);
            continue;
        }

        let collected = Resources {
            ore: entry.robots.ore as u8,
            clay: entry.robots.clay as u8,
            obsidian: entry.robots.obsidian as u8,
            geode: entry.robots.geode as u8,
        };

        if entry.resources.has_enough(&blueprint.geode_robot) {
            stack.push(Entry {
                time_left,
                resources: entry.resources
                    .subtract(&blueprint.geode_robot)
                    .add(&collected),
                robots: Robots {
                    geode: entry.robots.geode + 1,
                    ..entry.robots.clone()
                }
            });
            continue;
        }

        if entry.resources.has_enough(&blueprint.ore_robot) {
            stack.push(Entry {
                time_left,
                resources: entry.resources
                    .subtract(&blueprint.ore_robot)
                    .add(&collected),
                robots: Robots {
                    ore: entry.robots.ore + 1,
                    ..entry.robots.clone()
                }
            });
        }
        if entry.resources.has_enough(&blueprint.clay_robot) {
            stack.push(Entry {
                time_left,
                resources: entry.resources
                    .subtract(&blueprint.clay_robot)
                    .add(&collected),
                robots: Robots {
                    clay: entry.robots.clay + 1,
                    ..entry.robots.clone()
                }
            });
        }
        if entry.resources.has_enough(&blueprint.obsidian_robot) {
            stack.push(Entry {
                time_left,
                resources: entry.resources
                    .subtract(&blueprint.obsidian_robot)
                    .add(&collected),
                robots: Robots {
                    obsidian: entry.robots.obsidian + 1,
                    ..entry.robots.clone()
                }
            });
        }

        stack.push(Entry {
            time_left,
            resources: entry.resources.add(&collected),
            robots: entry.robots
        });
    }

    result.into()
}

fn main() {
    let path = Path::new("inputs/day19.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut result = 0;

    for (i, line) in reader.lines().enumerate() {
        let blueprint = parse_blueprint(&line.unwrap());

        println!("{}: {blueprint:?}", i + 1);

        let max_geodes = get_max_geode_count(blueprint);
        println!("max_geodes = {max_geodes}");

        let quality_level = (i + 1) * max_geodes as usize;
        result += quality_level;
    }

    println!("[Part 1] Sum of quality level of all of the blueprints is {}",
             result);
}
