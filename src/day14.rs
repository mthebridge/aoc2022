#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Content {
    Air,
    Rock,
    Sand,
}

fn fall_sand(grid: &[Vec<Content>], start_x: usize, start_y: usize) -> Option<(usize, usize)> {
    println!("Falling from ({}, {})", start_x, start_y);
    let next = grid[start_x][start_y..]
        .iter()
        .position(|c| *c != Content::Air);
    match next {
        Some(res) => {
            let y = res + start_y;
            println!("Blockage at ({}, {})", start_x, y);
            // Full.  try left.
            if grid[start_x - 1][y] == Content::Air {
                fall_sand(&grid, start_x - 1, y)
            } else if grid[start_x + 1][y] == Content::Air {
                fall_sand(&grid, start_x + 1, y)
            } else {
                // Sand ends up on the level above
                Some((start_x, y - 1))
            }
        }
        None => None,
    }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test14.txt")
    } else {
        include_str!("../inputs/day14.txt")
    };

    let mut max_x = 500;
    let mut max_y = 0;

    let edges = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|pos| {
                    let (x, y) = pos.split_once(",").unwrap();
                    let ret = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
                    max_x = ret.0.max(max_x);
                    max_y = ret.1.max(max_x);
                    ret
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    max_x += 1;
    max_y += 1;

    let mut grid = vec![vec![Content::Air; max_y]; max_x];

    for path in &edges {
        for edge in path.windows(2) {
            // Draw the rock.
            if edge[0].0 == edge[1].0 {
                for y in edge[0].1..=edge[1].1 {
                    let x = edge[0].0;
                    grid[x][y] = Content::Rock;
                }
            } else {
                assert_eq!(edge[0].1, edge[1].1);
                for x in edge[0].0..=edge[1].0 {
                    let y = edge[0].1;
                    grid[x][y] = Content::Rock;
                }
            }
        }
    }

    // Now simulate the falls.

    let mut part1 = 0;
    for idx in 1.. {
        match fall_sand(&grid, 500, 0) {
            Some((x, y)) => grid[x][y] = Content::Sand,
            None => {
                part1 = idx;
                break;
            }
        }
    }
    println!("Part 1: {}", part1);

    let part2 = 0;
    println!("Part 2: {}", part2)
}
