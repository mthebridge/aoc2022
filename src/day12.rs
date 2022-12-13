// The algorithm here is pretty poor.
// I didn't remember/look up Dijkstra - so I've implemented a poor approximation.
// Run in release mode if you want to actually get answers in a sensible timeframe!
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn unvisited_neighbours<'a, 'b>(
        &'a self,
        max_x: usize,
        max_y: usize,
        visited: &'b [Position],
    ) -> impl Iterator<Item = Position>
    where
        'b: 'a,
    {
        let mut neighbours = HashSet::with_capacity(4);
        let mut maybe_add = |pos: Position| {
            if !visited.contains(&pos) {
                neighbours.insert(pos);
            }
        };
        if self.x > 0 {
            maybe_add(Position {
                x: self.x - 1,
                ..*self
            });
        }
        if self.y > 0 {
            maybe_add(Position {
                y: self.y - 1,
                ..*self
            });
        }
        if self.x < max_x - 1 {
            maybe_add(Position {
                x: self.x + 1,
                ..*self
            });
        }
        if self.y < max_y - 1 {
            maybe_add(Position {
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
        path: &mut Vec<Position>,
        max_x: usize,
        max_y: usize,
        cache: &mut HashMap<Position, usize>,
    ) -> Option<usize> {
        match cache.get(self) {
            Some(hit) if *hit <= path.len() => {
                // We've been down this route before and we got here faster in another way.
                // So stop looking - no possible faster path here.
                None
            }
            _ => {
                if self == target {
                    // At the end!
                    Some(0)
                } else {
                    // Record that we got here in N steps, and then ad this node to the current path.
                    cache.insert(*self, path.len());
                    path.push(*self);
                    self.unvisited_neighbours(max_x, max_y, path)
                        .filter_map(|next| {
                            let self_height = heights.get(self).unwrap();
                            let next_height = heights.get(&next).unwrap();
                            if self_height + 1 >= *next_height {
                                // Can move here.  Recurse.
                                next.find_distance(
                                    heights,
                                    target,
                                    &mut path.clone(),
                                    max_x,
                                    max_y,
                                    cache,
                                )
                                .map(|x| x + 1)
                            } else {
                                // Can't go this way.
                                None
                            }
                        })
                        // Want the shortest route from all 4 neighbours.
                        .min()
                }
            }
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

    // Parse the heights
    let height_map = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                let height = match val {
                    'S' => 1,
                    'E' => 26,
                    x if x.is_ascii_lowercase() => (x as u8 - b'a') + 1,
                    _ => panic!("invalid character"),
                };
                (Position { x, y }, height)
            })
        })
        .collect::<HashMap<_, _>>();

    // Find the start and endpoints
    let start = find_position(input, 'S');
    let target = find_position(input, 'E');

    // Keep a cache of places we've already checked
    let cache = &mut HashMap::with_capacity(max_x * max_y);
    let part1 = start
        .find_distance(&height_map, &target, &mut vec![], max_x, max_y, cache)
        .unwrap();
    println!("Part 1: {}", part1);

    let part2 = height_map
        .iter()
        .filter_map(|(k, v)| if *v == 1 { Some(k) } else { None })
        .map(|k| {
            k.find_distance(&height_map, &target, &mut vec![], max_x, max_y, cache)
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap();
    println!("Part 2: {}", part2)
}
