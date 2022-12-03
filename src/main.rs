#![feature(slice_as_chunks)]
mod day01;
mod day02;
mod day03;

fn main() -> Result<(), String> {
    let day_num: u16 = std::env::args()
        .nth(1)
        .expect("No argument provided")
        .parse::<u16>()
        .map_err(|_| "Must pass numeric argument".to_string())?;
    match day_num {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        _ => panic!("Day not implemented"),
    }

    Ok(())
}