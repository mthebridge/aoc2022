type Position = (i64, i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Sensor {
    pub location: Position,
    pub closest_beacon: Position,
    pub closest_beacon_dist: u64,
}

#[inline]
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

    let sensors = input
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let location: Position = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let beacon: Position = (captures[3].parse().unwrap(), captures[4].parse().unwrap());
            Sensor {
                location,
                closest_beacon: beacon,
                closest_beacon_dist: distance(&beacon, &location),
            }
        })
        .collect::<Vec<_>>();

    let max_x = sensors
        .iter()
        .map(|sensor| {
            // The furthest we need to check is the eastmost beacon, plus twice its distance
            sensor.closest_beacon.0 + 2 * (sensor.closest_beacon_dist as i64)
        })
        .max()
        .unwrap();
    let min_x = sensors
        .iter()
        .map(|sensor| {
            // The furthest we need to check is the westmost beacon, minus twice its distance
            sensor.closest_beacon.0 - 2 * (sensor.closest_beacon_dist as i64)
        })
        .min()
        .unwrap();

    // Want to find all positions that are closer to any sensor than their matching beacon on the row.
    let y_row = if std::env::var("AOC_TEST").is_ok() {
        10
    } else {
        2_000_000
    };
    let part1 = (min_x..=max_x)
        .filter(|&x| {
            let this = (x, y_row);
            // dbg!(&this);
            sensors.iter().any(|sensor| {
                sensor.closest_beacon != this
                    && distance(&this, &sensor.location) <= sensor.closest_beacon_dist
        })})
        .count();
    // dbg!(&impossible_positions);

    println!("Part 1: {}", part1);

    let rangemax = if std::env::var("AOC_TEST").is_ok() {
        20
    } else {
        4_000_000
    };

    // The naive "test every point" is too slow...
    let part2 = (0..=rangemax)
        .filter_map(|x| {
            if x % 100 == 0 {
                println!("Loop {}", x);
            }
            match (0..=rangemax).find(|&y| {
                let this = (x, y);
                sensors
                    .iter()
                    .all(|sensor| distance(&this, &sensor.location) > sensor.closest_beacon_dist)
            }) {
                Some(y) => Some((x, y)),
                None => None,
            }
        })
        .next()
        .unwrap();

    dbg!(&part2);
    println!("Part 2: {}", part2.0 * 4_000_000 + part2.1)
}
