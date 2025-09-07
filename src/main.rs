use std::error::Error;

mod day;

fn main() -> Result<(), Box<dyn Error>> {
    day::d01::run()?;
    Ok(())
}
