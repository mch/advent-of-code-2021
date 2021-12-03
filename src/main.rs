use std::error::Error;

mod day1;
mod day2;
mod day3;

// Box<dyn Error>???
fn main() -> Result<(), Box<dyn Error>> {
    //day1::day1()
    //day2::day2()
    //day2::day2p2()
    day3::day3()
}

