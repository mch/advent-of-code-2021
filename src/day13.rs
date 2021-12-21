use crate::grid::{Grid, Point};
use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day13-input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
}

fn parse_dot_positions(lines: &mut std::str::Lines) -> Vec<Point> {
    let mut dots = Vec::new();
    let mut line = lines.next();
    while line != Some("") {
        println!("Processing line {}", line.unwrap());
        let numbers: Vec<&str> = line.unwrap().split(',').collect();

        if numbers.len() == 2 {
            // In Elm I could do Result.map2... does Rust have something similar?
            let x = numbers[0].parse::<usize>();
            let y = numbers[1].parse::<usize>();
            if x.is_ok() && y.is_ok() {
                dots.push(Point::new(x.unwrap(), y.unwrap()));
            } else {
                println!("Unable to parse dot position line {}", line.unwrap());
            }
        }
        line = lines.next(); // I don't like this, maybe just do a for loop and break
    }
    dots
}

fn parse_fold_instructions(lines: &mut std::str::Lines) -> Vec<Fold> {
    let mut folds: Vec<Fold> = Vec::new();
    for line in lines {
        let interesting_part = line.strip_prefix("fold along ");
        let parts: Vec<&str> = interesting_part.unwrap().split('=').collect();
        if parts.len() == 2 {
            let axis = match parts[0] {
                "x" => Some(Axis::X),
                "y" => Some(Axis::Y),
                _ => None
            };
            let position = parts[1].parse::<usize>();

            if axis.is_some() && position.is_ok() {
                folds.push(Fold { axis: axis.unwrap(), position: position.unwrap() });
            }
        } else {
            println!("Unable to parse fold line {}", line);
        }
    }
    folds
}

#[derive(Debug, PartialEq)]
enum Axis {
    X,
    Y
}

#[derive(Debug, PartialEq)]
struct Fold {
    axis: Axis,
    position: usize,
}

mod tests {
    use super::*;

    #[test]
    fn day13_parse_dot_positions() {
        let input = "6,10\n0,14\n9,10\n\n";
        let dots = parse_dot_positions(&mut input.lines());
        let expected: Vec<Point> = vec![Point::new(6, 10), Point::new(0, 14), Point::new(9, 10)];
        assert_eq!(expected, dots);
    }

    #[test]
    fn day13_parse_fold_instructions() {
        let input = "fold along y=7\nfold along x=5";
        let folds = parse_fold_instructions(&mut input.lines());
        let expected =
            vec![Fold { axis: Axis::Y, position: 7 }, Fold { axis: Axis::X, position: 5}];
        assert_eq!(expected, folds);
    }
}
