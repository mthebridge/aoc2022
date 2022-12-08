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
                    // Trees on the edge are always visible
                    x == 0 || x == max_x - 1 || y == 0 || y == max_y - 1 ||
                    // Check if all trees in any direction are shorter than this one.
                    (0..x).all(|i| heights[y][i] < this_height)
                        || (x + 1..max_x).all(|i| heights[y][i] < this_height)
                        || (0..y).all(|j| heights[j][x] < this_height)
                        || (y + 1..max_y).all(|j| heights[j][x] < this_height)
                }).count() // Count the number of visible trees in this row
        })
        .sum(); // Sum across all rows.
    let part2 = 0;
    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2)
}
