use itertools::Itertools;

// Ideally we would define our own type structure here and parse manually, but - life's too short.
// The packets are all (potentially nested) JSON lists of integers.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet(serde_json::Value);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use serde_json::Value;
        match (&self.0, &other.0) {
            (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
            (Value::Array(a), Value::Array(b)) => {
                // Compare element by element.
                std::iter::zip(a, b)
                    .map(|(a, b)| (Packet(a.clone()).cmp(&Packet(b.clone()))))
                    .find(|res| *res != std::cmp::Ordering::Equal)
                    .unwrap_or_else(|| a.len().cmp(&b.len()))
            }
            (Value::Array(_), Value::Number(_)) => {
                self.cmp(&Packet(Value::Array(vec![other.0.clone()])))
            }
            (Value::Number(_), Value::Array(_)) => {
                Packet(Value::Array(vec![self.0.clone()])).cmp(other)
            }
            _ => panic!("Invalid comparison"),
        }
    }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test13.txt")
    } else {
        include_str!("../inputs/day13.txt")
    };

    let pairs = input.split("\n\n").map(|pair| {
        pair.lines()
            .map(|line| Packet(serde_json::from_str(line).unwrap()))
            .collect_tuple::<(_, _)>()
            .unwrap()
    });

    let part1: usize = pairs
        .clone()
        .enumerate()
        .filter_map(|(idx, (a, b))| if a < b { Some(idx + 1) } else { None })
        .sum();
    println!("Part 1: {}", part1);

    let mut part2_pairs = pairs.flat_map(|(a, b)| vec![a, b]).collect::<Vec<_>>();
    let div_packets = vec![
        Packet(serde_json::from_str("[[2]]").unwrap()),
        Packet(serde_json::from_str("[[6]]").unwrap()),
    ];
    part2_pairs.append(&mut div_packets.clone());
    part2_pairs.sort();

    let part2: usize = part2_pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pkt)| {
            if div_packets.contains(pkt) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product();

    println!("Part 2: {}", part2)
}
