use crate::grid::{Grid, Point};
use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day13-input.txt").unwrap();
    let mut lines = input.lines();
    let dots = parse_dot_positions(&mut lines);
    let paper = convert_dots_to_grid(&dots);
    let folds = parse_fold_instructions(&mut lines);
    //println!("dots: {:?}", dots);
    //println!("folds: {:?}", folds);
    println!("Initial paper size: {}x{}", paper.width, paper.height);
    let mut folded_paper = paper;
    for next_fold in folds {
        folded_paper = match fold(&folded_paper, &next_fold) {
            Ok(paper) => {
                paper
            },
            Err(err) => {
                println!("Failed to fold paper: {}", err);
                break;
            }
        };
        println!("Folded paper size: {}x{}", folded_paper.width, folded_paper.height);
        let num_visible_dots = folded_paper.iter().fold(0, |count, point| {
            count + folded_paper.value(&point)
        });
        println!("Number of visible dots: {}", num_visible_dots);
    }
    println!("Folded result: \n{}", folded_paper);
    for y in 0..folded_paper.height {
        for x in 0..folded_paper.width {
            let v = folded_paper.value(&Point::new(x, y));
            let s = if v == 1 { "#" } else { " " };
            print!("{}", s);
        }
        print!("\n");
    }
}

fn parse_dot_positions(lines: &mut std::str::Lines) -> Vec<Point> {
    let mut dots = Vec::new();
    let mut line = lines.next();
    while line != Some("") {
        //println!("Processing line {}", line.unwrap());
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

fn convert_dots_to_grid(dots: &Vec<Point>) -> Grid {
    // Find grid size
    let mut size = Point::new(0, 0);
    for point in dots.iter() {
        if point.x > size.x {
            size.x = point.x;
        }
        if point.y > size.y {
            size.y = point.y
        }
    }
    //println!("Grid size: {:?}", size);

    // Grid evidently needs to be able to be created without a data arg...
    let placeholder: Vec<i32> = vec![0; (size.x + 1) * (size.y + 1)];
    //println!("Placeholder: {:?}", placeholder);
    let mut grid = Grid::new(&placeholder, size.x + 1, size.y + 1);
    // Create grid object
    for point in dots.iter() {
        //println!("Setting point {:?}", point);
        grid.set_value(point, 1);
    }
    grid
}

fn fold(input: &Grid, fold: &Fold) -> Result<Grid, String> {
    println!("Folding a paper sized {}x{} with fold {:?}", input.width, input.height, fold);
    let mut folded_size = Point::new(input.width, input.height);
    match fold.axis {
        Axis::X => {
            folded_size.x = fold.position;
        },
        Axis::Y => {
            folded_size.y = fold.position;
        }
    };
    println!("Folded size will be {}x{}", folded_size.x, folded_size.y);

    let data = vec![0; folded_size.x * folded_size.y];
    let mut grid = Grid::new(&data, folded_size.x, folded_size.y);
    //println!("folded paper: {:?}", grid);
    // update output grid...
    for point in input.iter() {
        let point_value = input.value(&point);
        if point_value == 0 {
            continue;
        }

        match fold.axis {
            // hmm these two arms are pretty much identical...
            Axis::X => {
                if point.x > fold.position {
                    let folded_point = Point::new(fold.position - (point.x - fold.position), point.y);
                    grid.set_value(&folded_point, 1);
                } else if point.x == fold.position {
                    // In the fold
                } else {
                    grid.set_value(&point, 1);
                }
            },
            Axis::Y => {
                if point.y > fold.position {
                    let folded_point = Point::new(point.x, fold.position - (point.y - fold.position));
                    grid.set_value(&folded_point, 1);
                } else if point.y == fold.position {
                    // In the fold
                } else {
                    //println!("setting value at point {:?}", point);
                    grid.set_value(&point, 1);
                }
            }
        }
    }

    Ok(grid)
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

    #[test]
    fn day13_positions_to_grid() {
        let input = "6,10\n0,14\n9,10\n\n";
        let dots = parse_dot_positions(&mut input.lines());
        let grid = convert_dots_to_grid(&dots);
        let mut expected_data = vec![0; 10*15];
        expected_data[106] = 1;
        expected_data[140] = 1;
        expected_data[109] = 1;
        let expected = Grid::new(&expected_data, 10, 15);
        println!("{}", expected);
        assert_eq!(expected, grid);
    }

    #[test]
    fn day13_test_fold() {
        let input_data = vec![1, 0, 0,
                              0, 0, 0,
                              1, 0, 1];
        let input_grid = Grid::new(&input_data, 3, 3);
        let expected_data = vec![1, 0, 1];
        let expected = Grid::new(&expected_data, 3, 1);
        assert_eq!(Ok(expected), fold(&input_grid, &Fold { axis: Axis::Y, position: 1}));
    }

    #[test]
    fn day13_test_folding_not_in_middle() {
        let input_data = vec![1, 0, 0,
                              0, 0, 0,
                              1, 0, 1,
                              0, 1, 0,
                              0, 1, 1];
        let input_grid = Grid::new(&input_data, 3, 5);
        let expected_data = vec![1, 0, 0,
                                 0, 0, 0,
                                 1, 1, 1];
        let expected = Grid::new(&expected_data, 3, 3);
        assert_eq!(Ok(expected), fold(&input_grid, &Fold { axis: Axis::Y, position: 3}));
    }
}
