pub fn run() {
    let input = include_str!("../inputs/day02.txt");

    let score: u32 = input
        .lines()
        .map(|l| {
            match l {
                "A X" => 4, // Rock-Rock draw: 1 + 3
                "A Y" => 8, // Paper beats Rock: 2 + 6
                "A Z" => 3, // Scissors lose to rock: 3 + 0
                "B X" => 1, // Rock loses to paper: 1 + 0
                "B Y" => 5, // Paper-Paper draw: 2 + 3
                "B Z" => 9, // Scissors beat paper: 3 + 6
                "C X" => 7, // Rock beats scissors: 1 + 6
                "C Y" => 2, // Paper loses scissors: 2 + 0
                "C Z" => 6, // Scissors-scissors draw: 3 + 3
                s => panic!("Invalid entry {}", s),
            }
        })
        .sum();

    println!("Part 1: {}", score);

    let real_score: u32 = input
        .lines()
        .map(|l| {
            match l {
                "A X" => 3, // Rock -> lose: choose scissors
                "A Y" => 4, // Rock -> draw: choose rock
                "A Z" => 8, // Rock -> win: choose paper
                "B X" => 1, // Paper -> lose: choose rock
                "B Y" => 5, // Paper -> draw: choose paper
                "B Z" => 9, // Paper -> win: choose scissors
                "C X" => 2, // Scissors -> lose: choose paper
                "C Y" => 6, // Scissors -> draw: choose scissors
                "C Z" => 7, // Scissors -> win: choose rock
                s => panic!("Invalid entry {}", s),
            }
        })
        .sum();

    println!("Part 2: {}", real_score)
}
