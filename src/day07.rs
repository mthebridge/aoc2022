use std::collections::HashMap;

fn get_path_str(path: &[&str]) -> String {
    path.join("/")
}

// Handle moving back up to a parent directory.
fn complete_child_traversal(sizes: &mut HashMap<String, u64>, cur_path: &mut Vec<&str>) {
    // Get total size of child
    let child_path = get_path_str(cur_path);
    let child_size: u64 = { *sizes.get(child_path.as_str()).unwrap() };
    // Move tracker back up to parent
    cur_path.pop().unwrap();
    let parent_path = get_path_str(cur_path);
    // Add the child size to parent.
    let parent_entry = sizes.entry(parent_path);
    parent_entry.and_modify(|s| {
        *s += child_size;
    });
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test07.txt")
    } else {
        include_str!("../inputs/day07.txt")
    };

    let mut cur_path = vec![];
    let mut sizes = HashMap::from([]);

    for line in input.lines() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("$") => {
                // Command
                match words.next().unwrap() {
                    "cd" => match words.next().unwrap() {
                        ".." => complete_child_traversal(&mut sizes, &mut cur_path),
                        subdir => {
                            // New subdirectory.  Set it as current and add it with empty size.
                            cur_path.push(subdir);
                            let ret = sizes.insert(get_path_str(&cur_path), 0);
                            // Check this path didn't already exist!
                            assert!(ret.is_none());
                        }
                    },
                    // Nothing to do here - we're about to get output.
                    "ls" => (),
                    _ => panic!("Unknown command"),
                }
            }
            Some("dir") => {
                // New directory - nothing to do.  Ignore the name.
                assert!(words.next().is_some());
            }
            Some(size_str) => {
                // New file - add size to current directory.
                let fsize: u64 = size_str.parse().unwrap();
                // Filename is irrelevant.
                assert!(words.next().is_some());
                sizes.entry(get_path_str(&cur_path)).and_modify(|s| {
                    *s += fsize;
                });
            }
            _ => panic!("Unexpected word"),
        }
        // Check we've parsed the whole line.
        assert_eq!(words.next(), None);
    }

    // Now do a final traverse up to the root to catch the final sizes.
    while !cur_path.is_empty() {
        complete_child_traversal(&mut sizes, &mut cur_path)
    }

    let part1: u64 = sizes.values().filter(|size| **size <= 100000).sum();
    println!("Part 1: {}", part1);
    let target = 30_000_000 - (70_000_000 - sizes.get("/").unwrap());
    let part2 = sizes.values().filter(|s| **s > target).min().unwrap();

    println!("Part 2: {}", part2)
}
