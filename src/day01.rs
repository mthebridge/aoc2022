pub fn run() {
    let input = include_str!("../inputs/day01.txt");

    let calorie_counts = input.split("\n\n").map(|lines| {
        lines
            .lines()
            .map(|l| l.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    println!("Part 1: {}", calorie_counts.clone().max().unwrap());

    let mut sorted = calorie_counts.collect::<Vec<_>>();
    sorted.sort();
    println!("Part 2: {}", sorted.iter().rev().take(3).sum::<u32>())
}
