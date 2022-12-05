pub fn run() {
    let input = include_str!("../inputs/day04.txt");

    // Uncomment for test
    //let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    let parse_range = |s: &str| {
        // Split on '-', and parse each side into an integer
        let (a, b) = s.split_once('-').expect("Bad range");
        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
    };

    let sections = input.lines().map(|l| {
        // Split each line at the comma, and parse the range each side
        let (a, b) = l.split_once(',').expect("Bad input");
        (parse_range(a), parse_range(b))
    });

    let part1 = sections
        .clone()
        .filter(|((a, b), (c, d))| {
            // Image to help visualise complete overlaps:
            // a..c..d..b  or  c..a..b..d
            (a >= c && b <= d) || (a <= c && b >= d)
        })
        .count();
    println!("Part 1: {}", part1);

    let part2 = sections
        .filter(|((a, b), (c, d))| {
            // Image to help visualise partial overlaps:
            // a..c..b..d  or  c..a..d..b, or the same as part1:
            // a..c..d..b  or  c..a..b..d
            // All the matters is a between c/d, or c between a/b
            (c <= b && c >= a) || (a >= c && a <= d)
        })
        .count();

    println!("Part 2: {}", part2)
}
