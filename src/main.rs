mod day01;

fn main() -> Result<(), String> {
    let day_num: u16 = std::env::args().nth(1).expect("No argument provided").parse::<u16>().map_err(|_| "Must pass numeric argument".to_string())?;
    match day_num {
        1 => day01::run(),
        _ => panic!("Day not implemented")
    }

    Ok(())

}
