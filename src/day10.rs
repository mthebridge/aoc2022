#[derive(Debug)]
enum Instr {
    Addx(i64),
    Noop,
}

impl Instr {
    fn parse(text: &str) -> Self {
        let mut words = text.split_whitespace();
        match words.next().unwrap() {
            "noop" => Instr::Noop,
            "addx" => Instr::Addx(words.next().unwrap().parse().unwrap()),
            _ => panic!("Bad instruction"),
        }
    }
}

pub fn run() {
    let input = if std::env::var("AOC_TEST").is_ok() {
        include_str!("../inputs/test10.txt")
    } else {
        include_str!("../inputs/day10.txt")
    };

    let mut instructions = input.lines().map(Instr::parse);

    // X register starts at 1
    let mut x = 1;
    let mut part1 = 0;
    let mut pending_add = None;

    // Keep track of the lit pixels
    let mut screen = [[false; 40]; 6];

    for cycle in 1usize.. {
        // part 1 counter
        if cycle % 40 == 20 {
            part1 += x * (cycle as i64);
        }
        // Part 2 - update the screen
        let cur_pixel = (cycle - 1) % 40;
        let cur_row = (cycle - 1) / 40;
        // Gross hack - we ought to check here for the next instruction being None, but that
        // messes up the counting...
        if cur_row >= 6 {
            break;
        };
        screen[cur_row][cur_pixel] = x.abs_diff(cur_pixel as i64) <= 1;

        if let Some(v) = pending_add {
            // Still on same instruction
            x += v;
            pending_add = None
        } else {
            match instructions.next() {
                Some(Instr::Addx(val)) => {
                    // Add takes 2 instructions to take effect.
                    pending_add = Some(val);
                }
                Some(Instr::Noop) => {}
                None => break,
            }
        }
    }
    println!("Part 1: {}", part1);

    // Draw the screen!
    println!("Part 2 image:");
    for row in screen {
        for cell in row {
            if cell {
                print!("#")
            } else {
                print!(" ")
            };
        }
        println!("")
    }
}
