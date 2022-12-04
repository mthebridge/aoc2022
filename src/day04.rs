pub fn run() {
    let input = include_str!("../inputs/day04.txt");

    // Uncomment for test
    //let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    let parse_range = |s: &str| {
        // Split on '-', and parse each side into an integer
        let (a, b) = s.split_once("-").expect("Bad range");
        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
    };

    let sections = input.lines().map(|l| {
        // Sllit each line at the comma, and aprse the range each side
        let (a, b) = l.split_once(",").expect("Bad input");
        (parse_range(a), parse_range(b))
    });

    let part1 = sections
        .clone()
        .filter(|((a, b), (c, d))| {
            // Image to help visualise:
            // a..c..d..b  or  c..a..b..d
            (a >= c && b <= d) || (a <= c && b >= d)
        })
        .count();
    println!("Part 1: {}", part1);

    let part2 = sections
        .filter(|((a, b), (c, d))| {
            // Image to help visualise:
            // a..c..b..d  or  c..a..d..b, or the same as part1:
            // a..c..d..b  or  c..a..b..d
            // So all that matters is that at least one of c/d must be between (inclusive) a/b, or vice versa
            (c <= b && c >= a) || (d >= a && d <= b) || (a >= c && a <= d) || (b >= c && b <= d)
        })
        .count();

    println!("Part 2: {}", part2)
}
