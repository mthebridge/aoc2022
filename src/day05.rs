use regex::Regex;

fn run_simulation(mut crates: Vec<Vec<char>>, instructions: &str, part2: bool) -> String {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for l in instructions.lines() {
        let captures = re.captures(l).unwrap();
        let amount: usize = captures[1].parse().unwrap();
        let src: usize = captures[2].parse().unwrap();
        let dest: usize = captures[3].parse().unwrap();
        if part2 {
            // Arrays are 0-indexed but the instructions are 1-indexed
            let src_stack = &mut crates[src - 1];
            let size = src_stack.len();
            let moved = &mut src_stack.drain(size - amount..size).collect();
            crates[dest - 1].append(moved);
        } else {
            for _ in 0..amount {
                // Pop off one at a time
                let val = crates[src - 1].pop().unwrap();
                crates[dest - 1].push(val);
            }
        }
    }
    crates.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn run() {
    let (start, instructions) = include_str!("../inputs/day05.txt")
        .split_once("\n\n")
        .unwrap();

    // Parse initial state.
    // Number of stacks is given by the last char in the last line of the starting state.
    let crate_size: usize = start
        .lines()
        .last()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();
    let mut crates: Vec<Vec<char>> = vec![vec![]; crate_size];
    // Iterate over the state in reverse order to build each stack up
    for line in start.lines().rev().skip(1) {
        //
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_uppercase() {
                // There are 4 chars ("[X] ") for each stack.  So take the index/4 for the stack number.
                crates[i / 4].push(c);
            }
        }
    }

    // Now parse the instructions.
    println!(
        "Part 1: {}",
        run_simulation(crates.clone(), instructions, false)
    );
    println!("Part 1: {}", run_simulation(crates, instructions, true));
}
