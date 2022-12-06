use std::collections::HashSet;

pub fn find_unique_substring_index(input: &[char], subsize: usize) -> Option<usize> {
    for (i, substr) in input.windows(subsize).enumerate() {
        let subset: HashSet<_> = substr.iter().collect();
        if subset.len() == subsize {
            return Some(i + subsize);
        }
    }
    None
}

pub fn run() {
    let input = include_str!("../inputs/day06.txt");
    let chars: Vec<_> = input.chars().collect();

    println!(
        "Part 1: {}",
        find_unique_substring_index(&chars, 4).unwrap()
    );
    println!(
        "Part 2: {}",
        find_unique_substring_index(&chars, 14).unwrap()
    )
}
