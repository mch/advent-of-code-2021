use std::fs;
use std::error::Error;

pub fn day2() -> Result<(), Box<dyn Error>> {
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

#[derive(Copy, Clone)]
struct SubmarineState {
    x_position: i32,
    depth: i32,
    aim: i32,
}

impl SubmarineState {
    fn new() -> SubmarineState {
        SubmarineState {
            x_position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn go_forward(&self, amount: i32) -> SubmarineState {
        SubmarineState {
            x_position: self.x_position + amount,
            depth: self.depth + self.aim * amount,
            ..*self
        }
    }

    fn go_down(&self, amount: i32) -> SubmarineState {
        SubmarineState {
            aim: self.aim + amount,
            ..*self
        }
    }

    fn go_up(&self, amount: i32) -> SubmarineState {
        SubmarineState {
            aim: self.aim - amount,
            ..*self
        }
    }

    fn process_command(&self, command: SubmarineCommand) -> SubmarineState {
        match command {
            SubmarineCommand::Forward(amount) => self.go_forward(amount),
            SubmarineCommand::Down(amount) => self.go_down(amount),
            SubmarineCommand::Up(amount) => self.go_up(amount),
            _ => *self
        }
    }
}

enum SubmarineCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
    None,
}

pub fn day2p2() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day2-input.txt").unwrap();

    // Split input into lines
    let line_iterator = input.lines();

    // Convert lines into commands
    let commands = line_iterator.map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 { return SubmarineCommand::None }

        let command = parts[1].parse().map(|amount| {
            match parts[0] {
                "forward" => SubmarineCommand::Forward(amount),
                "up" => SubmarineCommand::Up(amount),
                "down" => SubmarineCommand::Down(amount),
                _ => SubmarineCommand::None,
            }
        }).unwrap_or(SubmarineCommand::None);

        command
    });

    // Fold commands into position state
    let initial: SubmarineState = SubmarineState::new();
    let result = commands.fold(initial, |state, command| {
        state.process_command(command)
    });

    // Calculate result
    let horizontal = result.x_position;
    let depth = result.depth;
    let aim = result.aim;
    let final_result = horizontal * depth;
    println!("Final horizontal position: {}, depth: {}, aim: {}, product: {}", horizontal, depth, aim, final_result);

    Ok(())
}

