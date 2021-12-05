use std::fs;

pub fn puzzle1_and_2() -> () {
    let input: String = fs::read_to_string("data/day4-input.txt").unwrap();
    let (numbers_to_call, mut boards) = load_input(&input);

    let mut scores: Vec<u32> = Vec::new();
    for call in numbers_to_call {
        for board in &mut boards {
            board.mark(&call);
            if board.winning() {
                // println!("Called number {}", call);
                // println!("Winning board: {:?}", board.data);
                // println!("Marks: {:?}", board.marks);
                let score = board.unmarked_sum() * call;
                scores.push(score);
            }
        }
        // Remove boards that have already won
        boards = boards.into_iter().filter(|board| !board.winning()).collect();
    }

    println!("First board to win has a score of {}", scores[0]);
    println!("Last board to win has a score of {}", scores[scores.len() - 1]);
}

fn load_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let lines = input.clone().lines();

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

    (result.numbers_to_call, result.boards)
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
    marks: Vec<(usize, usize)>,
}

impl Board {
    fn new(data: Vec<Vec<u32>>) -> Board {
        Board {
            data,
            marks: Vec::new(),
        }
    }

    fn mark(&mut self, &number: &u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.data[i][j] == number {
                    self.marks.push((i,j));
                }
            }
        }
    }

    fn winning(&self) -> bool {
        for current_row in 0..5 {
            let mut count = 0;
            for (row, _) in &self.marks {
                if row == &current_row {
                    count = count + 1;
                }
            }
            if count == 5 {
                return true
            }
        }

        for current_column in 0..5 {
            let mut count = 0;
            for (_, column) in &self.marks {
                if column == &current_column {
                    count = count + 1;
                }
            }
            if count == 5 {
                return true
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for column in 0..5 {
                if !self.marks.contains(&(row, column)) {
                    sum = sum + self.data[row][column];
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_board() -> Board {
        let mut data: Vec<Vec<u32>> = Vec::new();
        for i in 0..5 {
            data.push(vec!(1 + 5 * i, 2 + 5 * i, 3 + 5 * i, 4 + 5 * i, 5 + 5 * i));
        }
        Board::new(data)
    }

    #[test]
    fn test_marking_row() {
        let mut board = create_test_board();
        assert_eq!(board.winning(), false);
        for i in 11..15 {
            board.mark(&i);
            assert_eq!(board.winning(), false);
        }
        board.mark(&15);
        assert_eq!(board.winning(), true);
    }

    #[test]
    fn test_marking_column() {
        let mut board = create_test_board();
        assert_eq!(board.winning(), false);
        board.mark(&2);
        assert_eq!(board.winning(), false);
        board.mark(&7);
        assert_eq!(board.winning(), false);
        board.mark(&12);
        assert_eq!(board.winning(), false);
        board.mark(&17);
        assert_eq!(board.winning(), false);
        board.mark(&22);
        assert_eq!(board.winning(), true);
    }

    #[test]
    fn test_marking_randmly() {
        // board 3 from the problem description
        let mut board = Board {
            data: vec!(
                vec!(14, 21, 17, 24, 4),
                vec!(10, 16, 15, 9, 19),
                vec!(18, 8, 23, 26, 20),
                vec!(22, 11, 13, 6, 5),
                vec!(2, 0, 12, 3, 7),
            ),
            marks: Vec::new(),
        };

        let calls: Vec<u32> = vec!(7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21);
        for call in calls {
            board.mark(&call);
            assert_eq!(board.winning(), false);
        }
        board.mark(&24);
        assert_eq!(board.winning(), true);
    }

    #[test]
    fn test_board_score() {
        let mut board = Board {
            data: vec!(
                vec!(14, 21, 17, 24, 4),
                vec!(10, 16, 15, 9, 19),
                vec!(18, 8, 23, 26, 20),
                vec!(22, 11, 13, 6, 5),
                vec!(2, 0, 12, 3, 7),
            ),
            marks: Vec::new(),
        };

        let calls: Vec<u32> = vec!(7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24);
        calls.iter().for_each(|call| board.mark(call));

        assert_eq!(board.unmarked_sum(), 188);
    }
}

