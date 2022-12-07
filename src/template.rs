pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/testNN.txt")
    } else {
        include_str!("../inputs/dayNN.txt")
    };

    let part1 = 0;
    let part2 = 0;
    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2)
}
