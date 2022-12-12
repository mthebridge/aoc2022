use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn neighbours(&self, max_x: usize, max_y: usize) -> impl Iterator<Item = Position> {
        let mut neighbours = HashSet::with_capacity(4);
        if self.x > 0 {
            neighbours.insert(Position {
                x: self.x - 1,
                ..*self
            });
        }
        if self.y > 0 {
            neighbours.insert(Position {
                y: self.y - 1,
                ..*self
            });
        }
        if self.x < max_x - 1 {
            neighbours.insert(Position {
                x: self.x + 1,
                ..*self
            });
        }
        if self.y < max_y - 1 {
            neighbours.insert(Position {
                y: self.y + 1,
                ..*self
            });
        }
        neighbours.into_iter()
    }

    fn find_distance(
        &self,
        heights: &HashMap<Position, u8>,
        target: &Position,
        visited: &mut HashSet<Position>,
        max_x: usize,
        max_y: usize,
    ) -> Option<usize> {
        if self == target {
            // At the end!
            println!("Target");
            Some(0)
        } else {
            let is_new = visited.insert(*self);
            self
                .neighbours(max_x, max_y)
                    .map(|next| {
                        let self_height = heights.get(self).unwrap();
                        let next_height = heights.get(&next).unwrap();
                        // Use cached value if been here before
                        if !is_new {
                            // Been this way before.  Ignore
                            // println!("Backtrack:  ({}, {})", next.x, next.y);
                            None
                        }
                        else if self_height + 1 >= *next_height {
                            // Not been here.
                            println!("Trying ({}, {})", next.x, next.y);
                            next.find_distance(
                                heights,
                                target,
                                &mut visited.clone(),
                                max_x,
                                max_y,
                            ).map(|x| x + 1)
                        } else {
                            // Can't go this way.
                            // println!("Blocked:  ({}, {})", next.x, next.y);
                            None
                        }
                    })
                    .flatten()
                    .min()
            }
        }

}

fn find_position(input: &str, needle: char) -> Position {
    let y = input
        .lines()
        .position(|line| line.contains(needle))
        .unwrap();
    let x = input
        .lines()
        .find(|line| line.contains(needle))
        .unwrap()
        .chars()
        .position(|c| c == needle)
        .unwrap();
    Position { x, y }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test12.txt")
    } else {
        include_str!("../inputs/day12.txt")
    };

    // Calculate grid size.
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();

    let height_map = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                let height = match val {
                    'S' => 1,
                    'E' => 26,
                    x if x.is_ascii_lowercase() => (x as u8 - 'a' as u8) + 1,
                    _ => panic!("invalid character"),
                };
                (Position { x, y }, height)
            })
        })
        .collect::<HashMap<_, _>>();

    // Starting at Position, try all neighbours.

    let start = find_position(input, 'S');
    let target = find_position(input, 'E');
    let mut visited = HashSet::with_capacity(max_x * max_y);
    let part1 = start.find_distance(&height_map, &target, &mut visited, max_x, max_y).unwrap();
    println!("Part 1: {}", part1);

    let part2 = 0;
    println!("Part 2: {}", part2)
}
