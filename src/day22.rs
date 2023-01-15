use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Turn {
    CounterClockwise,
    Clockwise,
}

impl Direction {
    fn turn(self, to: Turn) -> Self {
        use Turn::*;
        use Direction::*;

        match self {
            Up => if to == CounterClockwise { Left } else { Right },
            Right => if to == CounterClockwise { Up } else { Down },
            Down => if to == CounterClockwise { Right } else { Left },
            Left => if to == CounterClockwise { Down } else { Up },
        }
    }

    fn score(self) -> usize {
        use Direction::*;

        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Step(usize),
    Turn(Turn),
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    let (mut i, mut j) = (0, 0);
    for b in line.bytes() {
        if b == b'L' || b == b'R' {
            let count = line[i..j].parse().unwrap();
            instructions.push(Instruction::Step(count));
            (i, j) = (j + 1, j + 1);

            let dir = if b == b'L' { 
                Turn::CounterClockwise
            } else { 
                Turn::Clockwise
            };
            instructions.push(Instruction::Turn(dir));
        } else {
            j += 1;
        }
    }
    instructions.push(Instruction::Step(line[i..].parse().unwrap()));

    instructions
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(self, dir: Direction) -> Position {
        use Direction::*;

        match dir {
            Up => Position { y: self.y - 1, ..self },
            Right => Position { x: self.x + 1, ..self },
            Down => Position { y: self.y + 1, ..self },
            Left => Position { x: self.x - 1, ..self },
        }
    }
}

fn parse_walls<I>(lines: &mut I) -> HashSet<Position>
where I: Iterator<Item = std::io::Result<String>> {
    lines
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| l
            .unwrap()
            .bytes()
            .collect::<Vec<u8>>()
            .into_iter()
            .enumerate()
            .filter_map(|(x, b)| if b == b'#' { Some(x) } else { None }))
        .enumerate()
        .flat_map(|(y, inner)| inner.map(move |x| (y, x)))
        .map(|(y, x)| Position { x, y })
        .collect()
}

struct Face {
    x: usize,
    y: usize,
    up: (usize, Direction),
    right: (usize, Direction),
    down: (usize, Direction),
    left: (usize, Direction),
}

const FACE_BEGIN: usize = 0;
const FACE_SIZE: usize = 50;

impl Face {
    fn to_global(&self, pos: Position) -> Position {
        Position {
            x: self.x * FACE_SIZE + pos.x,
            y: self.y * FACE_SIZE + pos.y
        }
    }

    fn get_next(&self, pos: Position, dir: Direction) -> (usize, Position, Direction) {
        use Direction::*;

        let (next_face, next_dir) = match dir {
            Up => self.up,
            Right => self.right,
            Down => self.down,
            Left => self.left,
        };

        let next_pos = transition(pos, dir, next_dir);

        (next_face, next_pos, next_dir)
    }
}

fn transition(pos: Position, from: Direction, to: Direction) -> Position {
    use Direction::*;

    match from {
        Up => {
            match to {
                Up => Position { y: FACE_SIZE - 1, ..pos },
                Right => Position { x: FACE_BEGIN, y: pos.x },
                _ => panic!("unexpected transition given"),
            }
        },
        Right => {
            match to {
                Right => Position { x: FACE_BEGIN, ..pos },
                Up => Position { x: pos.y, y: FACE_SIZE - 1 },
                Down => Position { x: pos.y, y: FACE_BEGIN },
                Left => Position { x: FACE_SIZE - 1, y: FACE_SIZE - 1 - pos.y },
            }
        },
        Down => {
            match to {
                Down => Position { y: FACE_BEGIN, ..pos },
                Left => Position { x: FACE_SIZE - 1, y: pos.x },
                _ => panic!("unexpected transition given"),
            }
        },
        Left => {
            match to {
                Left => Position { x: FACE_SIZE - 1, ..pos },
                Right => Position { x: FACE_BEGIN, y: FACE_SIZE - 1 - pos.y },
                Down => Position { x: pos.y, y: FACE_BEGIN },
                _ => panic!("unexpected transition given"),
            }
        },
    }
}

fn would_step_from_face(pos: Position, dir: Direction) -> bool {
    use Direction::*;

    match dir {
        Up => pos.y == FACE_BEGIN,
        Right => pos.x == FACE_SIZE - 1,
        Down => pos.y == FACE_SIZE - 1,
        Left => pos.x == FACE_BEGIN,
    }
}

fn calculate_password(
    faces: &[Face; 6], instructions: &[Instruction], walls: &HashSet<Position>
) -> usize {
    let (mut face, mut pos, mut dir)
        = (0, Position { x: 0, y: 0 }, Direction::Right);

    for instr in instructions {
        match *instr {
            Instruction::Turn(to) => {
                dir = dir.turn(to)
            },
            Instruction::Step(count) => {
                for _ in 0..count {
                    let (
                        next_face, next_pos, next_dir
                    ) = if would_step_from_face(pos, dir) {
                        faces[face].get_next(pos, dir)
                    } else {
                        (face, pos.step(dir), dir)
                    };

                    if walls.contains(&faces[next_face].to_global(next_pos)) {
                        break;
                    }

                    (face, pos, dir) = (next_face, next_pos, next_dir);
                }
            },
        }
    }

    let global_pos = faces[face].to_global(pos);

    1000 * (global_pos.y + 1)
        + 4 * (global_pos.x + 1)
        + dir.score()
}

fn part1(instructions: &[Instruction], walls: &HashSet<Position>) {
    const FACES: [Face; 6] = [
        Face {
            x: 1, y: 0,
            up: (4, Direction::Up),
            right: (1, Direction::Right),
            down: (2, Direction::Down),
            left: (1, Direction::Left),
        },
        Face {
            x: 2, y: 0, 
            up: (1, Direction::Up),
            right: (0, Direction::Right),
            down: (1, Direction::Down),
            left: (0, Direction::Left),
        },
        Face {
            x: 1, y: 1,
            up: (0, Direction::Up),
            right: (2, Direction::Right),
            down: (4, Direction::Down),
            left: (2, Direction::Left),
        },
        Face {
            x: 0, y: 2,
            up: (5, Direction::Up),
            right: (4, Direction::Right),
            down: (5, Direction::Down),
            left: (4, Direction::Left),
        },
        Face {
            x: 1, y: 2,
            up: (2, Direction::Up),
            right: (3, Direction::Right),
            down: (0, Direction::Down),
            left: (3, Direction::Left),
        },
        Face {
            x: 0, y: 3,
            up: (3, Direction::Up),
            right: (5, Direction::Right),
            down: (3, Direction::Down),
            left: (5, Direction::Left),
        },
    ];

    let password = calculate_password(&FACES, instructions, walls);
    println!("[Part 1] The final password value is {}", password);
}

fn part2(instructions: &[Instruction], walls: &HashSet<Position>) {
    const FACES: [Face; 6] = [
        Face {
            x: 1, y: 0,
            up: (5, Direction::Right),
            right: (1, Direction::Right),
            down: (2, Direction::Down),
            left: (3, Direction::Right),
        },
        Face {
            x: 2, y: 0,
            up: (5, Direction::Up),
            right: (4, Direction::Left),
            down: (2, Direction::Left),
            left: (0, Direction::Left),
        },
        Face {
            x: 1, y: 1,
            up: (0, Direction::Up),
            right: (1, Direction::Up),
            down: (4, Direction::Down),
            left: (3, Direction::Down),
        },
        Face {
            x: 0, y: 2,
            up: (2, Direction::Right),
            right: (4, Direction::Right),
            down: (5, Direction::Down),
            left: (0, Direction::Right),
        },
        Face {
            x: 1, y: 2,
            up: (2, Direction::Up),
            right: (1, Direction::Left),
            down: (5, Direction::Left),
            left: (3, Direction::Left),
        },
        Face {
            x: 0, y: 3,
            up: (3, Direction::Up),
            right: (4, Direction::Up),
            down: (1, Direction::Down),
            left: (0, Direction::Down),
        },
    ];

    let password = calculate_password(&FACES, instructions, walls);
    println!("[Part 2] The final password value for the map \
              folded in a cube is {}", password);
}

fn main() {
    let path = Path::new("inputs/day22.txt");
    let mut reader = match File::open(path) {
        Err(e) => panic!("could not open input file at {}: {}", path.display(), e),
        Ok(file) => BufReader::new(file),
    };

    let walls = parse_walls(&mut reader
        .by_ref()
        .lines());

    let instrs = parse_instructions(&reader
        .lines()
        .next()
        .unwrap()
        .unwrap());

    part1(&instrs, &walls);
    part2(&instrs, &walls);
}
