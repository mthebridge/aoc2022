#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Content {
    Air,
    Rock,
    Sand,
}

#[allow(dead_code)]
// Print grid for debugging
fn print_grid(grid: &[Vec<Content>]) {
    for y in 0..grid[0].len() {
        for col in grid {
            let c = match col[y] {
                Content::Air => ' ',
                Content::Rock => '#',
                Content::Sand => 'o',
            };
            print!("{}", c);
        }
        println!();
    }
}

// Emulate a fall of a single grain.  Returns the resting place, or None if it would fall
// off the bottom.
fn fall_sand(grid: &[Vec<Content>], start_x: usize, start_y: usize) -> Option<(usize, usize)> {
    let next = grid[start_x][start_y..]
        .iter()
        .position(|c| *c != Content::Air);
    match next {
        Some(res) => {
            let y = res + start_y;
            // Full.  Try the left, then right.
            if grid[start_x - 1][y] == Content::Air {
                fall_sand(grid, start_x - 1, y)
            } else if grid[start_x + 1][y] == Content::Air {
                fall_sand(grid, start_x + 1, y)
            } else {
                // Sand ends up on the level above.
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
                    let (x, y) = pos.split_once(',').unwrap();
                    let ret = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
                    max_x = ret.0.max(max_x);
                    max_y = ret.1.max(max_y);
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
                let yrange = edge[0].1.min(edge[1].1)..=edge[0].1.max(edge[1].1);
                for y in yrange {
                    let x = edge[0].0;
                    grid[x][y] = Content::Rock;
                }
            } else {
                assert_eq!(edge[0].1, edge[1].1);
                let xrange = edge[0].0.min(edge[1].0)..=edge[0].0.max(edge[1].0);
                for x in xrange {
                    let y = edge[0].1;
                    grid[x][y] = Content::Rock;
                }
            }
        }
    }

    // Now simulate the falls.
    let mut part1_grid = grid.clone();
    let mut part1 = 0;
    for idx in 0.. {
        match fall_sand(&part1_grid, 500, 0) {
            Some((x, y)) => part1_grid[x][y] = Content::Sand,
            None => {
                part1 = idx;
                break;
            }
        }
    }

    println!("Part 1: {}", part1);

    let mut part2_grid = grid.clone();

    // For part2, add the floor, and an extra few columns.
    for _ in 1..max_y {
        part2_grid.push(vec![Content::Air; max_y]);
    }
    for column in part2_grid.iter_mut() {
        column.push(Content::Air);
        column.push(Content::Rock);
    }
    let mut part2 = 0;
    for idx in 0.. {
        match fall_sand(&part2_grid, 500, 0) {
            Some((500, 0)) => {
                part2 = idx + 1;
                break;
            }
            Some((x, y)) => part2_grid[x][y] = Content::Sand,
            None => panic!("Impossible"),
        }
    }
    println!("Part 2: {}", part2)
}
