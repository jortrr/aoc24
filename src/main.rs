use std::error::Error;

mod day;
mod file;

fn main() -> Result<(), Box<dyn Error>> {
    day::d01::run()?;
    day::d02::run()?;
    Ok(())
}
