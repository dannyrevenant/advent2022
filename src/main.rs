use std::error::Error;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn main() -> Result<(), Box<dyn Error>> {
    day_01::answer();
    day_02::answer();
    day_03::answer();
    day_04::answer();
    day_05::answer()?;
    day_06::answer()?;
    day_07::answer()?;
    day_08::answer()?;

    Ok(())
}
