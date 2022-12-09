use std::collections::HashSet;

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
// - if same row or column, move one step closer on the other axis
// - otherwise, move diagonally closer
fn update_knot(first: Position, second: Position) -> Position {
    let mut new_second = second;
    if second.y == first.y {
        // Same y, one step closer on x
        if second.x > first.x + 1 {
            new_second.x -= 1;
        } else if second.x < first.x - 1 {
            new_second.x += 1;
        }
    } else if second.x == first.x {
        // Same x, one step closer on y
        if second.y > first.y + 1 {
            new_second.y -= 1;
        } else if second.y < first.y - 1 {
            new_second.y += 1;
        }
    } else if second.y.abs_diff(first.y) > 1 ||
        // More than two away and not in same row or column
        // Move both x and y one closer.
        second.x.abs_diff(first.x) > 1 {
        if second.y < first.y {
            new_second.y += 1
        } else {
            new_second.y -= 1
        }

        if second.x < first.x {
            new_second.x += 1
        } else {
            new_second.x -= 1
        }
    } else {
        // Nothing to do - stay put.
    }
    new_second

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
                let previous = if knot == 0 { self.head } else { self.knots[knot - 1] };
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
    let mut rope: Rope = Default::default();
    rope.knots = vec![Default::default(); num_knots];

    for line in input.lines() {
        let (dir, count_str) = line.split_once(" ").unwrap();
        let count: usize = count_str.parse().unwrap();
        match dir {
            "U" => rope.move_head(Direction::Up, count, num_knots),
            "D" => rope.move_head(Direction::Down, count, num_knots),
            "L" => rope.move_head(Direction::Left, count, num_knots),
            "R" => rope.move_head(Direction::Right, count, num_knots),
            _ => panic!("Invalid direction")
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

    let part1 = simulate_rope(&input, 1);
    println!("Part 1: {}", part1);

    let part2 = simulate_rope(&input, 9);
    println!("Part 2: {}", part2)
}
