// Calculate the "score" for a part2 viewing range.
fn score_range<R, P>(range: &mut R, filter: P) -> usize
where
    R: Iterator<Item = usize> + Clone,
    P: Fn(usize) -> bool,
{
    // We want to count the trees shorter than this in order, but add 1 only if there's another tree after that
    let max_distance = range.clone().count();
    let short_count = range.take_while(|i| filter(*i)).count();
    short_count + if max_distance == short_count { 0 } else { 1 }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test08.txt")
    } else {
        include_str!("../inputs/day08.txt")
    };

    // calculate grid size.
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();
    // Parse
    let heights = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // iterate over each entry.
    let part1: usize = heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, &this_height)| {
                    // Check if all trees in any direction are shorter than this one.
                    (0..x).all(|i| heights[y][i] < this_height)
                        || (x + 1..max_x).all(|i| heights[y][i] < this_height)
                        || (0..y).all(|j| heights[j][x] < this_height)
                        || (y + 1..max_y).all(|j| heights[j][x] < this_height)
                })
                .count() // Count the number of visible trees in this row
        })
        .sum(); // Sum across all rows.

    println!("Part 1: {}", part1);

    // Calculate "score".  Similar to part1.
    let part2 = heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &this_height)| {
                    // Check the score for each direction. Score is number in order in that direction
                    // that are smaller than this one.
                    score_range(&mut (0..x).rev(), |i| heights[y][i] < this_height)
                        * score_range(&mut (x + 1..max_x), |i| heights[y][i] < this_height)
                        * score_range(&mut (0..y).rev(), |j| heights[j][x] < this_height)
                        * score_range(&mut (y + 1..max_y), |j| heights[j][x] < this_height)
                })
                .max() // Find the largest score in this row
                .unwrap()
        })
        .max() // Find the largest score in all rows
        .unwrap();

    println!("Part 2: {}", part2)
}
