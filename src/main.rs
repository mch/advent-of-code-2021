use std::fs;
use std::error::Error;

// Box<dyn Error>???
fn main() -> Result<(), Box<dyn Error>> {
    day1()
}

////////////////////////////////////////////////////////////////////////////////
// Day 1

fn day1() -> Result<(), Box<dyn Error>> {
    // How can I clone a String inside a Result<String, io::Error>,
    // where io::Error doesn't implement Clone?
    let input: String = fs::read_to_string("aoc-2021-puzzle1.txt").unwrap();
    let input1 = input.clone();
    let input2 = input.clone();
    let input3 = input.clone();
    let ups = count_larger_measurements(input1);
    println!("num single measurement ups: {}", ups);

    let window_ups = count_larger_measurement_window(input2);
    println!("num windowsed measurement ups: {}", window_ups);

    let numbers: Vec<u32> = parse_lines(&input3);

    // I don't understand why numbers_iter has to be mutable. Is it because next()
    // mutates self? If so, how do for loops etc get away with it? Do they make
    // mutable copies?
    let mut numbers_iter = numbers.iter();
    let first_number: Option<&u32> = numbers_iter.next();
    let accumulator: (u32, u32) = (0, first_number.unwrap_or(&0).clone()); //yolo
    let result = numbers_iter.fold(accumulator, |accumulator: (u32, u32), number: &u32| {
        let (ups, previous) = accumulator;
        if *number > previous {
             (ups+1, *number)
        } else {
            (ups, *number)
        }
    });
    let (ups, previous) = result;
    println!("fold result: ({}, {})", ups, previous);

    Ok(())
}

fn count_larger_measurements(input: String) -> u32 {
    let mut ups: u32 = 0;
    let mut lines = input.lines();
    let mut previous_item: u32 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        let item: u32 = line.parse().unwrap();
        if item > previous_item {
            ups = ups + 1;
        }
        previous_item = item;
    }

    ups
}

fn count_larger_measurement_window(input: String) -> u32 {
    let mut ups: u32 = 0;
    let mut lines = input.lines();
    let mut item0: u32 = lines.next().unwrap().parse().unwrap();
    let mut item1: u32 = lines.next().unwrap().parse().unwrap();
    let mut item2: u32 = lines.next().unwrap().parse().unwrap();
    let mut previous_window = item0 + item1 + item2;
    //println!("w: {} + {} + {} = {}", item0, item1, item2, previous_window);

    for line in lines {
        item0 = item1;
        item1 = item2;
        item2 = line.parse().unwrap();
        let window = item0 + item1 + item2;
        if window > previous_window {
            ups = ups + 1;
        }
        //println!("w: {} + {} + {} = {}, {}", item0, item1, item2, window, ups);
        previous_window = window;
    }

    ups
}

fn parse_lines(input: &str) -> Vec<u32>{
    let mut ugh: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parse_result = line.parse();
        parse_result.map_or_else(|e| {
            println!("Unable to parse {} as a number: {}", line, e);
        }, |number| { ugh.push(number); });
    }
    ugh
}

////////////////////////////////////////////////////////////////////////////////
// Day 2
////////////////////////////////////////////////////////////////////////////////

fn day2() {

}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    // Day 1 tests
    #[test]
    fn test_parsing_ints() {
        assert_eq!(parse_lines("1\n34\n43\n"), vec![1, 34, 43]);
    }

    #[test]
    fn test_parsing_not_ints() {
        assert_eq!(parse_lines("1\nHi\n43\n"), vec![1, 43]);

    }

    // Day 2 tests

}
