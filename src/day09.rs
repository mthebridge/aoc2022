use std::{collections::HashSet, cmp::Ordering};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Rope {
    pub head: Position,
    pub knots: Vec<Position>,
    pub visited: HashSet<Position>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Update a single knot to move based on the position of the previous knot in the chain.
// Rules are:
// - only move if more than one away in any direction
// - if so, move 1 closer in all directions where not in same column.
fn update_knot(first: Position, second: Position) -> Position {
    // Nothing to do unless more than one away in some direction.
    if second.x.abs_diff(first.x) > 1 || second.y.abs_diff(first.y) > 1 {
        let step_size = |a: i64, b| {
            match a.cmp(&b) {
                Ordering::Equal => 0,
                Ordering::Less => 1,
                Ordering::Greater => -1,
            }
        };
        Position {
            x: second.x + step_size(second.x, first.x),
            y: second.y + step_size(second.y, first.y),
        }
    } else {
        second
    }
}

impl Rope {
    // Move the rope head, and then update all the knots to follow
    pub fn move_head(&mut self, dir: Direction, steps: usize, num_knots: usize) {
        for _ in 0..steps {
            match dir {
                Direction::Up => self.head.y += 1,
                Direction::Down => self.head.y -= 1,
                Direction::Right => self.head.x += 1,
                Direction::Left => self.head.x -= 1,
            }
            for knot in 0..num_knots {
                let previous = if knot == 0 {
                    self.head
                } else {
                    self.knots[knot - 1]
                };
                self.knots[knot] = update_knot(previous, self.knots[knot]);
                // println!("Head: ({},{}), knots: ({:?})", self.head.x, self.head.y, self.knots);
            }
            // Mark the tail position as visited
            self.visited.insert(self.knots[num_knots - 1]);
        }
    }
}

// Solve the puzzle - create a rope, and move it according to the input.
// Return the required output - number of spaces visited by the rope's last knot.
pub fn simulate_rope(input: &str, num_knots: usize) -> usize {
    let mut rope = Rope {
        knots: vec![Default::default(); num_knots],
        ..Default::default()
    };

    for line in input.lines() {
        let (dir, count_str) = line.split_once(' ').unwrap();
        let count: usize = count_str.parse().unwrap();
        match dir {
            "U" => rope.move_head(Direction::Up, count, num_knots),
            "D" => rope.move_head(Direction::Down, count, num_knots),
            "L" => rope.move_head(Direction::Left, count, num_knots),
            "R" => rope.move_head(Direction::Right, count, num_knots),
            _ => panic!("Invalid direction"),
        }
    }
    rope.visited.len()
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test09.txt")
    } else {
        include_str!("../inputs/day09.txt")
    };

    let part1 = simulate_rope(input, 1);
    println!("Part 1: {}", part1);

    let part2 = simulate_rope(input, 9);
    println!("Part 2: {}", part2)
}
