use std::collections::HashSet;


type Position = (i64, i64);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Sensor {
    pub location: Position,
    pub closest_beacon: Position,
    pub closest_beacon_dist: u64
}

fn distance(this: &Position, other: &Position) -> u64 {
    (this.0 - other.0).abs() as u64 + (this.1 - other.1).abs() as u64
}


pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test15.txt")
    } else {
        include_str!("../inputs/day15.txt")
    };

    let re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    let mut clear_set = HashSet::new();
    let sensors = input
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let location: Position = (
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
            );
            let beacon: Position = (
                captures[3].parse().unwrap(),
                captures[4].parse().unwrap(),
            );
            // Build the set of clear spaces between the sensor and beacon.
            let dist = distance(&beacon, &location);
            let idist = dist as i64;
            dbg!(idist);
            clear_set.extend((location.0 - idist..location.0 + idist).flat_map(|x| {
                // if x % 100 == 0 {
                //     println!("Onto row {}", x);
                // }
                (location.1 - idist..location.1 + idist).filter_map(move |y| {
                    if distance(&(x, y), &location) <= dist && ((x,y)) != beacon {
                        Some((x, y))
                    } else { None }
                })
            }));

            Sensor {
                location,
                closest_beacon: beacon,
                closest_beacon_dist: dist,
            }
        })
        .collect::<Vec<_>>();

    dbg!(&sensors);

    // Want to find all positions that are closer to any sensor than their matching beacon on the row.
    let y_row = if std::env::var("AOC_TEST").is_ok() {
        10
    } else {
        2_000_000
    };
    let part1 = clear_set.iter().filter(|&pos| pos.1 ==  y_row).collect::<HashSet<_>>();

    //  dbg!(&part1);

    println!("Part 1: {}", part1.len());

    let rangemax = if std::env::var("AOC_TEST").is_ok() {
        20
    } else {
        4_000_000
    };

    // The naive "test every point" is too slow ()
    let mut part2 = (0,0);
    'outer: for x in 0..=rangemax {
        if x % 100 == 0 {
            println!("Onto ({}, 0)", x);
        }
        for y in 0..=rangemax {
            if sensors.iter().all(|sensor| sensor.closest_beacon != (x, y)) && !clear_set.contains(&(x, y)) {
                part2 = (x, y);
                break 'outer
            }
        }
    }
    dbg!(&part2);
    println!("Part 2: {}", part2.0 * 4_000_000 + part2.1)
}
