use std::collections::HashSet;

pub fn find_unique_substring_index(input: &[u8], subsize: usize) -> Option<usize> {
    input
        .windows(subsize)
        .position(|substr| substr.iter().collect::<HashSet<_>>().len() == subsize)
        .map(|i| i + subsize)
}

pub fn run() {
    let input = include_bytes!("../inputs/day06.txt");
    // let chars: Vec<_> = input.chars().collect();

    println!("Part 1: {}", find_unique_substring_index(input, 4).unwrap());
    println!(
        "Part 2: {}",
        find_unique_substring_index(input, 14).unwrap()
    )
}
