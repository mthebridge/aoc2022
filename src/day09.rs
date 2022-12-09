use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Rope {
    pub head: Position,
    pub tails: Vec<Position>,
    pub visited: HashSet<Position>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_knot(first: Position, second: Position) -> Position {
    let mut new_second = second;
    if second.y == first.y {
        // Same line, one step closer
        if second.x > first.x + 1 {
            new_second.x -= 1;
        } else if second.x < first.x - 1 {
            new_second.x += 1;
        }
    } else if second.x == first.x {
        // Same line, one step closer
        if second.y > first.y + 1 {
            new_second.y -= 1;
        } else if second.y < first.y - 1 {
            new_second.y += 1;
        }
    } else if second.y.abs_diff(first.y) > 1 ||
        second.x.abs_diff(first.x) > 1 {
        // More than two away
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
    }
    new_second

}

impl Rope {
    pub fn move_head(&mut self, dir: Direction, steps: usize, num_knots: usize) {
        for _ in 0..steps {
            match dir {
                Direction::Up => self.head.y += 1,
                Direction::Down => self.head.y -= 1,
                Direction::Right => self.head.x += 1,
                Direction::Left => self.head.x -= 1,
            }
            for knot in 0..num_knots {
                let previous = if knot == 0 { self.head } else { self.tails[knot - 1] };
                self.tails[knot] = update_knot(previous, self.tails[knot]);
                // println!("Head: ({},{}), Tails: ({:?})", self.head.x, self.head.y, self.tails);
            }
            // Mark this position as visited
            self.visited.insert(self.tails[num_knots - 1]);
        }
    }
}

pub fn simulate_rope(input: &str, num_knots: usize) -> usize {
    let mut rope: Rope = Default::default();
    rope.tails = vec![Default::default(); num_knots];

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

    // Start the head and tail at (0,0)


    let part1 = simulate_rope(&input, 1);
    println!("Part 1: {}", part1);

    let part2 = simulate_rope(&input, 9);
    println!("Part 2: {}", part2)
}
