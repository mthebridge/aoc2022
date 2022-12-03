fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        panic!("Invalid char {}", c)
    }
}

// Finds the single unique character in a set of strings.
// Assumes that there is a single unique character.
// If more than one match, a single answer will be returned.
fn find_unique_ascii_char(strings: &[&str]) -> char {
    let mut minimal = strings.iter().next().expect("Empty input").to_string();
    for s in strings.iter() {
        minimal = minimal.chars().filter(|c| s.contains(*c)).collect();
    }

    minimal.chars().next().unwrap()
}

pub fn run() {
    let input = include_str!("../inputs/day03.txt");

    let sacks = input.lines();
    let part1: u32 = sacks
        .map(|sack| {
            // Split the sack in half.  We can assume even length.
            let (upper, lower) = sack.split_at(sack.len() / 2);
            // Find any char in the first that matches the second half
            let uniq = find_unique_ascii_char(&[upper, lower]);
            char_to_priority(uniq)
        })
        .sum();

    println!("Part 1: {}", part1);

    // Annoyingly you can't easily iterate in chunks over an arbitrary
    // iterator, so we have to collect into something we can then slice over,
    // since slices *do* support chunking.
    let lines = input.lines().collect::<Vec<_>>();
    let groups = lines.as_slice().chunks(3);
    let part2: u32 = groups
        .map(|group_sacks| {
            let uniq = find_unique_ascii_char(group_sacks);
            char_to_priority(uniq)
        })
        .sum();
    println!("Part 2: {}", part2)
}
