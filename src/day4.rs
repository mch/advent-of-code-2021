use std::fs;
use std::error::Error;

pub fn puzzle1() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day4-input.txt").unwrap();
    load_input2(&input);

    Ok(())
}

pub fn puzzle2() -> Result<(), Box<dyn Error>> {

    Ok(())
}

fn load_input(input: &str) -> () {
    let mut lines = input.clone().lines();

    let numbers_to_call: Vec<u32> = lines.next().unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", numbers_to_call);

    lines.next(); // blank line...

    let boards: Vec<Board> = Vec::new();

    let board_line: Vec<u32> = lines.next().unwrap()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    println!("{:?}", board_line);
}

fn load_input2(input: &str) -> () {
    let mut lines = input.clone().lines();

    let result = lines.fold(Accumulator {
        state: State::ReadNumbers,
        numbers_to_call: Vec::new(),
        boards: Vec::new(),
        partial_board: Vec::new()
    }, |mut accumulator, line| {
        match accumulator.state {
            State::ReadNumbers => {
                accumulator.numbers_to_call = line.split(',')
                    .map(|x| x.parse::<u32>())
                    .filter(|x| x.is_ok())
                    .map(|x| x.unwrap())
                    .collect();
                accumulator.state = State::ReadBlank;
            }
            State::ReadBlank => {
                accumulator.state = State::ReadBoard;
            }
            State::ReadBoard => {
                let board_line: Vec<u32> = line.split(' ')
                    .map(|x| x.parse::<u32>())
                    .filter(|x| x.is_ok()) // filter out extra spaces
                    .map(|x| x.unwrap())
                    .collect();
                accumulator.partial_board.push(board_line);

                if accumulator.partial_board.len() == 5 {
                    accumulator.state = State::ReadBlank;
                    accumulator.boards.push(Board::new(accumulator.partial_board));
                    accumulator.partial_board = Vec::new();
                }
            }
        }
        accumulator
    });
}

#[derive(Clone, Debug)]
enum State {
    ReadNumbers,
    ReadBlank,
    ReadBoard,
}

#[derive(Clone, Debug)]
struct Accumulator {
    state: State,
    numbers_to_call: Vec<u32>,
    boards: Vec<Board>,
    partial_board: Vec<Vec<u32>>
}

#[derive(Clone, Debug)]
struct Board {
    data: Vec<Vec<u32>>,
}

impl Board {
    fn new(data: Vec<Vec<u32>>) -> Board {
        Board {
            data
        }
    }
}
