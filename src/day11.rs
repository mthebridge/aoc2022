use std::sync::atomic::Ordering;
use std::{cell::RefCell, sync::atomic::AtomicU32};

use itertools::Itertools;

struct Monkey {
    pub worry_items: RefCell<Vec<u64>>,
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub divisor: u64,
    pub true_target: usize,
    pub false_target: usize,
    pub inspection_count: AtomicU32,
}

fn run_part(input: &str, part2: bool) -> u64 {
    let monkey_regex = regex::Regex::new(
        r"Monkey \d+:
\s*Starting items: ((?:\d+,? ?)*)
\s*Operation: new = (.*)
\s*Test: divisible by (\d+)
\s*If true: throw to monkey (\d+)
\s*If false: throw to monkey (\d+)",
    )
    .unwrap();

    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|section| {
            let matches = monkey_regex.captures(section).expect("Invalid format");
            let item_str = &matches[1];
            let op_str = &matches[2];
            let divisor = matches[3].parse().unwrap();
            let true_target = matches[4].parse().unwrap();
            let false_target = matches[5].parse().unwrap();
            Monkey {
                worry_items: RefCell::new(
                    item_str.split(", ").map(|s| s.parse().unwrap()).collect(),
                ),
                operation: parse_operation_str(op_str),
                divisor,
                true_target,
                false_target,
                inspection_count: AtomicU32::new(0),
            }
        })
        .collect();

    // To avoid overflows, do everything modulo the LCM of all the divisor tests.
    let lcm: u64 = monkeys.iter().map(|m| m.divisor).product();
    // Now run the rounds
    for _ in 0..(if part2 { 10000 } else { 20 }) {
        run_round(&monkeys, part2, lcm)
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| m.inspection_count.load(Ordering::Relaxed))
        .sorted()
        .rev();

    inspection_counts.next().unwrap() as u64 * inspection_counts.next().unwrap() as u64
}

fn parse_operation_str(op_str: &str) -> Box<dyn Fn(u64) -> u64> {
    // This is fiddly, because we need to dynamically define the closure
    let (operand1, operation, operand2) = op_str.split_whitespace().collect_tuple().unwrap();
    let convert_operand = |x: &str| {
        if x == "old" {
            None
        } else {
            Some(x.parse::<u64>().unwrap())
        }
    };
    let op1 = convert_operand(operand1);
    let op2 = convert_operand(operand2);
    match (op1, operation, op2) {
        (Some(a), "+", None) | (None, "+", Some(a)) => Box::new(move |x| x + a),
        (Some(a), "*", None) | (None, "*", Some(a)) => Box::new(move |x| x * a),
        (None, "+", None) => Box::new(|x| x * 2),
        (None, "*", None) => Box::new(|x| x * x),
        _ => panic!("Invalid oepration string"),
    }
}

fn run_round(monkeys: &[Monkey], part2: bool, lcm: u64) {
    // Iterate over each monkey and run the logic.
    for monkey in monkeys {
        for item in monkey.worry_items.borrow_mut().drain(..) {
            monkey.inspection_count.fetch_add(1, Ordering::Relaxed);
            let new_worry = {
                let tmp = (*monkey.operation)(item % lcm);
                if !part2 {
                    tmp / 3
                } else {
                    tmp
                }
            };
            let target = if new_worry % monkey.divisor == 0 {
                monkey.true_target
            } else {
                monkey.false_target
            };
            monkeys[target].worry_items.borrow_mut().push(new_worry)
        }
    }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test11.txt")
    } else {
        include_str!("../inputs/day11.txt")
    };

    println!("Part 1: {}", run_part(input, false));
    println!("Part 2: {}", run_part(input, true));
}
