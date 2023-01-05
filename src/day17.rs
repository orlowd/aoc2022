use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

type Position = (usize, usize);

const SHAPES: [&[Position]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (1, 0), (0, 1), (1, 1)]
];

const WIDTH: usize = 7;
type Chamber = Vec<[bool; WIDTH]>;

fn intersects(rock: &[(usize, usize)], pos: Position, chamber: &Chamber) -> bool {
    rock.iter().any(|p| {
        let x = pos.0 + p.0;
        let y = pos.1 + p.1;

        x >= 7 || chamber[y][x]
    })
}

fn place(rock: &[(usize, usize)], pos: Position, chamber: &mut Chamber) -> usize {
    for p in rock {
        let x = pos.0 + p.0;
        let y = pos.1 + p.1;

        chamber[y][x] = true;
    }

    extend_for_next(chamber)
}

fn extend_for_next(chamber: &mut Chamber) -> usize {
    let mut y = 0;

    for ln in chamber.iter().rev() {
        if ln.iter().any(|&x| x) {
            break;
        } else {
            y += 1;
        }
    }

    let highest = chamber.len() - (y + 1);

    const MAX_REQUIRED: usize = 7;
    let ext = MAX_REQUIRED - y;
    chamber.resize(chamber.len() + ext, [false; WIDTH]);

    highest
}

const SPAWN_X: usize = 2;
const SPAWN_Y: usize = 3;

fn place_rock(
    shape: &[Position],
    highest: usize,
    jet_index: &mut usize,
    chamber: &mut Chamber,
    moves: &Vec<Move>
) -> usize {
    let mut x = SPAWN_X;
    let mut y = highest;

    loop {
        let mv = &moves[*jet_index % moves.len()];
        *jet_index += 1;

        x = match mv {
            Move::Left if x != 0 => {
                let next_x = x - 1;

                if !intersects(shape, (next_x, y), chamber) {
                    next_x
                } else {
                    x
                }
            },
            Move::Right if x != WIDTH - 1 => {
                let next_x = x + 1;

                if !intersects(shape, (next_x, y), chamber) {
                    next_x
                } else {
                    x
                }
            },
            _ => x,
        };

        if y == 0 || intersects(shape, (x, y - 1), chamber) {
            break;
        } else {
            y -= 1;
        }
    }

    place(shape, (x, y), chamber)
}

// 7 is 3 (vertical spawn distance) + 4 (highest shape)
const RESERVED_LINES: usize = 7;

fn part_1(moves: &Vec<Move>) {
    const NUM_ROCKS: usize = 2022;

    let mut chamber: Chamber = vec![[false; WIDTH]; RESERVED_LINES];
    let mut highest: Option<usize> = None;
    let mut jet = 0;

    for i in 0..NUM_ROCKS {
        highest = Some(place_rock(
            SHAPES[i % SHAPES.len()],
            highest.map_or_else(|| SPAWN_Y, |h| h + 1 + SPAWN_Y),
            &mut jet,
            &mut chamber,
            moves,
        ));
    }

    println!("[Part 1] After {} rocks have stopped falling \
              the tower will be {} units tall",
             NUM_ROCKS, highest.unwrap() + 1);
}

const SEARCH_HEIGHT: usize = 100;

fn find_cycle(
    chamber: &mut Chamber,
    range: Range<usize>,
    highest: &mut usize,
    jet_index: &mut usize,
    moves: &Vec<Move>
) -> (usize, usize, usize) {
    type State = (usize, usize, [[bool; WIDTH]; SEARCH_HEIGHT]);
    let mut cycles = HashMap::<State, (usize, usize)>::new();

    let total_count = range.end;

    for i in range {
        *highest = place_rock(
            SHAPES[i % SHAPES.len()],
            *highest + 1 + SPAWN_Y,
            jet_index,
            chamber,
            moves,
        );

        let mut last_rows = [[false; WIDTH]; SEARCH_HEIGHT];
        let iter = chamber[0..=*highest]
            .iter()
            .rev()
            .take(SEARCH_HEIGHT);

        for (i, row) in iter.enumerate() {
            last_rows[i] = *row;
        }

        let cur_state = (i % SHAPES.len(), *jet_index % moves.len(), last_rows);
        if let Some((prev_height, prev_i)) = cycles.insert(cur_state, (*highest, i)) {
            let height_diff = *highest - prev_height;
            let count_diff = i - prev_i;

            let count_remaining = total_count - (i + 1);
            let repeats_count = count_remaining / count_diff;
            let remaining = count_remaining % count_diff;
            let height_from_repeats = height_diff * repeats_count;

            let continue_at = i + 1;

            return (continue_at, remaining, height_from_repeats);
        }
    }

    panic!("could not find a cycle!");
}

fn part_2(moves: &Vec<Move>) {
    const NUM_ROCKS: usize = 1_000_000_000_000;

    let mut chamber: Chamber = vec![[false; WIDTH]; RESERVED_LINES];
    let mut highest: Option<usize> = None;
    let mut jet = 0;

    for i in 0..(SEARCH_HEIGHT + RESERVED_LINES) {
        highest = Some(place_rock(
            SHAPES[i % SHAPES.len()],
            highest.map_or_else(|| SPAWN_Y, |h| h + 1 + SPAWN_Y),
            &mut jet,
            &mut chamber,
            moves,
        ));
    }

    let mut highest = highest.unwrap();

    let (cont_at, remaining, height_from_repeats) = find_cycle(
        &mut chamber,
        (SEARCH_HEIGHT + RESERVED_LINES)..NUM_ROCKS,
        &mut highest,
        &mut jet,
        moves
    );

    for i in cont_at..(cont_at + remaining) {
        highest = place_rock(
            SHAPES[i % SHAPES.len()],
            highest + 1 + SPAWN_Y,
            &mut jet,
            &mut chamber,
            moves,
        );
    }

    println!("[Part 2] After {} rocks have stopped falling \
              the tower will be {} units tall",
             NUM_ROCKS, highest + height_from_repeats + 1);
}

fn main() {
    let path = Path::new("inputs/day17.txt");
    let input = read_to_string(path).unwrap();

    let moves: Vec<_> = input.trim_end().bytes().into_iter().map(|mv| {
        use self::Move::*;

        match mv {
            b'<' => Left,
            b'>' => Right,
            b => panic!("Unexpected input movement given: {}!", b as char),
        }
    }).collect();

    part_1(&moves);
    part_2(&moves);
}
