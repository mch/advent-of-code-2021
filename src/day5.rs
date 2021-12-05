use std::fs;
use std::error::Error;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day1-input.txt").unwrap();
}

mod tests {

    #[test]
    fn test_day5_foo() {
        println!("A test!");
        assert_eq!(0, 1);
    }

    #[test]
    fn test_day5_parse_line() {
        let input = "48,233 -> 48, 456";

        // See the example code in the std::str::FromStr docs
    }
}
