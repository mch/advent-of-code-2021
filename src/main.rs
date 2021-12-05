use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

// Box<dyn Error>???
fn main() -> Result<(), Box<dyn Error>> {
    //day1::day1()
    //day2::day2()
    //day2::day2p2()
    //day3::puzzle1()
    //day3::puzzle2()
    //day4::puzzle1_and_2()
    day5::puzzle1()
}

