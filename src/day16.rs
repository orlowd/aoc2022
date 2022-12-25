use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

type Index = usize;
type Indexes = HashMap<String, Index>;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    leads_to: Vec<Index>,
}

type Valves = Vec<Valve>;

fn parse_valves<I>(lines: I) -> (Valves, Indexes)
where I: Iterator<Item = std::io::Result<String>> {
    let mut valves: Valves = Vec::new();
    let mut indexes: Indexes = HashMap::new();

    let mut valve_leads_to: Vec<Vec<String>> = vec![];

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();

        let name = iter
            .nth(1)
            .unwrap()
            .to_string();

        let flow_rate = iter
            .nth(2)
            .unwrap()
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse().unwrap();

        let leads_to: Vec<_> = iter
            .skip(4)
            .map(|s| s.trim_end_matches(',').to_string())
            .collect();

        indexes.insert(name, i);
        valves.push(Valve {
            flow_rate,
            leads_to: Vec::with_capacity(leads_to.len())
        });
        valve_leads_to.push(leads_to);
    }

    for (i, leads_to) in valve_leads_to.into_iter().enumerate() {
        for next in leads_to {
            valves[i].leads_to.push(indexes[&next]);
        }
    }

    (valves, indexes)
}

fn find_min(from: Index, to: Index, valves: &Valves) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    const MOVE_COST: u32 = 1;

    queue.push((Reverse(0), from));
    visited.insert(from);

    while let Some((Reverse(cost), cur)) = queue.pop() {
        if cur == to {
            return cost;
        }

        valves[cur].leads_to
            .iter()
            .filter(|&&next| visited.insert(next))
            .for_each(|&next| queue.push((Reverse(cost + MOVE_COST), next)));
    }

    panic!("No path found between nodes {from} and {to}!");
}

fn distances<'a, I>(
    indexes: I,
    start_idx: &'a Index,
    valves: &Valves
) -> HashMap<(Index, Index), u32>
where I: Iterator<Item = &'a Index> + Clone {
    indexes
        .chain([start_idx])
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (&from, &to)| {
            let distance = find_min(from, to, valves);

            acc.insert((from, to), distance);
            acc.insert((to, from), distance);

            acc
        })
}

fn simulate(
    valves: &Valves,
    start_idx: Index,
    time_limit: u32,
    flowing: &BTreeSet<Index>,
    dists: &HashMap<(Index, Index), u32>
) -> HashMap::<BTreeSet<Index>, u32> {
    const OPEN_COST: u32 = 1;

    let mut result = HashMap::<BTreeSet<Index>, u32>::new();
    let mut queue = VecDeque::from([(start_idx, BTreeSet::new(), time_limit, 0)]);

    while let Some((cur, open, time_left, flow)) = queue.pop_front() {
        result
            .entry(open.clone())
            .and_modify(|v| *v = (*v).max(flow))
            .or_insert(flow);

        let closed = flowing.difference(&open);

        for &next in closed {
            let time_to_open = dists[&(cur, next)] + OPEN_COST;
            if time_to_open >= time_left {
                continue;
            }

            let new_time_left = time_left - time_to_open;
            let new_flow = flow + valves[next].flow_rate * new_time_left;

            let mut new_open = open.clone();
            new_open.insert(next);

            queue.push_back((next, new_open, new_time_left, new_flow));
        }
    }

    result
}

fn main() {
    let path = Path::new("src/day16.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let (valves, indexes) = parse_valves(reader.lines());

    let flowing: BTreeSet<_> = valves
        .iter()
        .enumerate()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(index, _)| index)
        .collect();

    let start_idx = indexes[&"AA".to_string()];

    let dists = distances(flowing.iter(), &start_idx, &valves);

    const PART1_LIMIT: u32 = 30;
    let result = *simulate(&valves, start_idx, PART1_LIMIT, &flowing, &dists)
        .values()
        .max()
        .unwrap();
    println!("[Part 1] The most pressure you can release is {}", result);

    const PART2_LIMIT: u32 = 26;
    let result = simulate(&valves, start_idx, PART2_LIMIT, &flowing, &dists)
        .iter()
        .combinations(2)
        .filter(|comb| comb[0].0.is_disjoint(comb[1].0))
        .map(|comb| comb[0].1 + comb[1].1)
        .max()
        .unwrap();
    println!("[Part 2] The most pressure you can release with an elephant \
              helping you is {}", result);
}
