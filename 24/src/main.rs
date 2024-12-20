use std::fmt::Display;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

#[derive(Debug)]
struct BadArgument(usize);
impl Display for BadArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There are not {} days in a month !", self.0)
    }
}
impl std::error::Error for BadArgument {}

// TODO: Gracefully prompt day/variant
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exercises = [
        vec![d1::run_1, d1::run_2],
        vec![d2::run_1, d2::run_2],
        vec![d3::run_1, d3::run_2],
        vec![d4::run_1, d4::run_2],
        vec![d5::run_1, d5::run_2],
    ];

    let mut args = std::env::args().skip(1);
    let arg = args.next().unwrap();
    let day: usize = arg.parse()?;
    if !(day > 0 && day <= 31) {
        return Err(Box::new(BadArgument(day)));
    }

    let arg = args.next().unwrap();
    let variant: usize = arg.parse()?;
    if variant == 0 {
        return Err(Box::new(BadArgument(variant)));
    }
    let input = std::fs::read_to_string(format!("inputs/d{}_{}", day, variant))?;
    println!("{}", exercises[day - 1][variant - 1](input));
    Ok(())
}
