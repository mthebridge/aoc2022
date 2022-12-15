type Position = (i64, i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Sensor {
    pub location: Position,
    pub closest_beacon: Position,
    pub closest_beacon_dist: u64,
}

#[inline]
fn distance(this: &Position, other: &Position) -> u64 {
    (this.0 - other.0).unsigned_abs() + (this.1 - other.1).unsigned_abs()
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
            sensors.iter().any(|sensor| {
                sensor.closest_beacon != this
                    && distance(&this, &sensor.location) <= sensor.closest_beacon_dist
            })
        })
        .count();

    println!("Part 1: {}", part1);

    let rangemax = if std::env::var("AOC_TEST").is_ok() {
        20
    } else {
        4_000_000
    };

    // The naive "test every point" is too slow...
    let mut x = 0;
    let mut y = 0;
    let mut hidden = None;
    while y <= rangemax {
        let this = (x as i64, y as i64);
        let xstep = sensors
            .iter()
            .filter_map(|sensor| {
                let dist = distance(&this, &sensor.location);
                if dist <= sensor.closest_beacon_dist {
                    // We are at most (beacon dist - this.1) from this.0. Work out which side, and then
                    // Move as far as we can out of this range.
                    let half_range =
                        sensor.closest_beacon_dist - sensor.location.1.abs_diff(this.1);
                    let nextx = if sensor.location.0 > this.0 {
                        // Sensor in front.  Jump forward by x-dist to location, then by the x-distance on the otherside
                        // (Which is the total beacon range of this sensor minus its  coord )
                        (sensor.location.0 - this.0) as u64 + half_range + 1
                    } else {
                        // Sensor is behind us. - Move forward the remainder
                        half_range - (this.0 - sensor.location.0) as u64 + 1
                    };
                    Some(nextx)
                } else {
                    None
                }
            })
            // We can skip over as many point as the furthest beacon.
            .max();
        match xstep {
            None => {
                hidden = Some((x, y));
                break;
            }
            Some(xval) => {
                if x + xval <= rangemax {
                    x += xval;
                } else {
                    y += 1;
                    x = 0;
                }
            }
        }
    }

    let part2 = hidden.unwrap();
    dbg!(&part2);
    println!("Part 2: {}", part2.0 as u64 * 4_000_000 + part2.1 as u64)
}
