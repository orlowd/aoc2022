use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Data {
    sensor: Position,
    beacon: Position,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: isize,
    to: isize,
}

fn coalesce_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut result: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(back) = result.last_mut() {
            if range.from <= back.to {
                back.to = back.to.max(range.to);
            } else {
                result.push(range);
            }
        } else {
            result.push(range);
        }
    }

    result
}

fn get_hor_ranges(y: isize, beacons: &Vec<Data>, bounds: Option<Range>) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();

    for beacon in beacons {
        let sensor = beacon.sensor;
        let beacon = beacon.beacon;
        let mht = beacon.manhattan(&sensor);
        
        let dist = mht as isize - sensor.y.abs_diff(y) as isize;
        if dist <= 0 {
            continue;
        }

        let mut range = Range {
            from: sensor.x - dist,
            to: sensor.x + dist + 1,
        };

        if let Some(Range{ from, to }) = bounds {
            range.from = range.from.clamp(from, to);
            range.to = range.to.clamp(from + 1, to + 1);
        }

        ranges.push(range);
    }

    ranges.sort_unstable_by(|l, r| {
        if l.from == r.from {
            l.to.cmp(&r.to)
        } else {
            l.from.cmp(&r.from)
        }
    });

    coalesce_ranges(ranges)
}

fn main() {
    let path = Path::new("inputs/day15.txt");
    let reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let mut beacons: Vec<Data> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();

        let sensor_x = iter
            .nth(2).unwrap()
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse().unwrap();
        let sensor_y = iter
            .next().unwrap()
            .trim_start_matches("y=")
            .trim_end_matches(':')
            .parse().unwrap();

        let beacon_x = iter
            .nth(4).unwrap()
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse().unwrap();
        let beacon_y = iter
            .next().unwrap()
            .trim_start_matches("y=")
            .parse().unwrap();

        beacons.push(
            Data {
                sensor: Position { x: sensor_x, y: sensor_y },
                beacon: Position { x: beacon_x, y: beacon_y },
            }
        );
    }

    const ROW_TO_CHECK: isize = 2_000_000;

    let ranges = get_hor_ranges(ROW_TO_CHECK, &beacons, None);

    let num_pos = ranges
        .into_iter()
        .fold(0, |acc, rng| acc + (rng.to - rng.from - 1));

    println!("[Part 1] The number of positions that cannot contain \
              a beacon in the row {} is {}", ROW_TO_CHECK, num_pos);

    const LOWER_BOUND: isize = 0;
    const UPPER_BOUND: isize = 4_000_000;

    const X_MULTIPLIER: isize = 4_000_000;

    let tun_freq = (LOWER_BOUND..=UPPER_BOUND)
        .map(|y| {
            (get_hor_ranges(
                y, &beacons,
                Some( Range { from : LOWER_BOUND, to : UPPER_BOUND })
            ), y)
        })
        .skip_while(|(ranges, _)| ranges.len() <= 1)
        .map(|(ranges, y)| {
            let x = ranges.first().unwrap().to;

            x * X_MULTIPLIER + y
        }).next().unwrap();

    println!("[Part 2] The tuning frequency of the distress beacon \
              is {}", tun_freq);
}
