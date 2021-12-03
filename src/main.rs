use std::fs;
use std::error::Error;

// Box<dyn Error>???
fn main() -> Result<(), Box<dyn Error>> {
    //day1()
    day2();
    day2p2()
}

////////////////////////////////////////////////////////////////////////////////
// Day 1

fn day1() -> Result<(), Box<dyn Error>> {
    // How can I clone a String inside a Result<String, io::Error>,
    // where io::Error doesn't implement Clone?
    let input: String = fs::read_to_string("aoc-day1-input.txt").unwrap();
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

fn day2() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day2-input.txt").unwrap();

    // Split input into lines
    let line_iterator = input.lines();

    // Convert lines into command tuples (command, amount): (string, number)
    let commands = line_iterator.map(parse_line);

    // Fold commands into position state
    let initial_position = (0, 0); // Tuple of (horizontal, depth)
    let position = commands.fold(initial_position, |(h, d), (command, amount)| {
        match command {
            "forward" => (h + amount, d),
            "down" => (h, d + amount),
            "up" => (h, d - amount),
            _ => (h, d)
        }
    });

    // Calculate result
    let (horizontal, depth) = position;
    let result = horizontal * depth;
    println!("Final horizontal position: {}, depth: {}, product: {}", horizontal, depth, result);

    Ok(())
}

fn parse_line(line: &str) -> (&str, i32) {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 2 {return ("none", 0)}

    let command: &str = parts[0];
    let amount: i32 = parts[1].parse().unwrap();

    (command, amount)
}

struct SubmarineState {
    x_position: i32,
    depth: i32,
    aim: i32,
}

enum SubmarineCommand {
    Forward,
    Up,
    Down,
    None,
}

fn day2p2() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day2-input.txt").unwrap();

    // Split input into lines
    let line_iterator = input.lines();

    // Convert lines into command tuples (command, amount): (string, number)
    let commands = line_iterator.map(parse_line);

    // Fold commands into position state
    let initial: SubmarineState = SubmarineState {
        x_position: 0,
        depth: 0,
        aim: 0,
    };
    let result = commands.fold(initial, |state, (command, amount)| {
        match command {
            "forward" => SubmarineState {
                x_position: state.x_position + amount,
                depth: state.depth + state.aim * amount,
                ..state
            },
            "down" => SubmarineState {
                aim: state.aim + amount,
                ..state
            },
            "up" => SubmarineState {
                aim: state.aim - amount,
                ..state
            },
            _ => state
        }
    });

    // Calculate result
    let horizontal = result.x_position;
    let depth = result.depth;
    let aim = result.aim;
    let final_result = horizontal * depth;
    println!("Final horizontal position: {}, depth: {}, aim: {}, product: {}", horizontal, depth, aim, final_result);

    Ok(())
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
