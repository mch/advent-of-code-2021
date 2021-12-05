use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day5-input.txt").unwrap();
    let segments: Vec<LineSegment> = input.lines()
        .map(|line| line.parse::<LineSegment>().unwrap())
        .collect();
    println!("Segments: {:?}", segments);

    //let mut points = HashMap::new();
    for segment in segments {
        // if segment.is_horizontal() {
            
        // } else if segment.is_vertical() {
            
        // }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').map(|x| x.trim()).collect();

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> LineSegment {
        LineSegment { a, b }
    }

    fn iter(&self) -> LineSegmentIter {
        LineSegmentIter::new(self)
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split("->").map(|x| x.trim()).collect();

        let a = points[0].parse::<Point>()?;
        let b = points[1].parse::<Point>()?;

        Ok(LineSegment { a, b })
    }
}

#[derive(Debug)]
struct LineSegmentIter {
    // TODO I feel like this should store an Rc<LineSegment> or something, but I'm just
    // copying it for now.
    segment: LineSegment,
    direction: Point, // vector really
    current: Point,
    done: bool, // cheesy, there's probably a better way
}

impl LineSegmentIter {
    fn new(segment: &LineSegment) -> LineSegmentIter {
        // Only handling horizontal or vertical lines for now.
        let direction = Point::new(segment.b.x - segment.a.x, segment.b.y - segment.a.y);
        let magnitude = ((direction.x as f64).powi(2) + (direction.y as f64).powi(2)).sqrt();
        let normed = Point::new(direction.x / magnitude as i32, direction.y / magnitude as i32);
        LineSegmentIter {
            segment: *segment,
            direction: normed,
            current: segment.a,
            done: false,
        }
    }
}

impl Iterator for LineSegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current;
            if result == self.segment.b {
                self.done = true;
            } else {
                self.current.x += self.direction.x;
                self.current.y += self.direction.y;
            }
            Some(result)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_day5_parse_point() {
        let input = "48,233";
        assert_eq!(Ok(Point::new(48, 233)), input.parse::<Point>());
    }

    // Not important right now
    // #[test]
    // fn test_day5_parse_point_empty() {
    //     let input = "";
    //     assert_eq!(Ok(Point::new(48, 233)), input.parse::<Point>());
    // }

    // #[test]
    // fn test_day5_parse_point_half() {
    //     let input = "48";
    //     assert_eq!(Ok(Point::new(48, 35)), input.parse::<Point>());
    // }

    // #[test]
    // fn test_day5_parse_point_float() {
    //     let input = "48,45.5";
    //     assert_eq!(Ok(Point::new(48, 35)), input.parse::<Point>());
    // }

    #[test]
    fn test_day5_parse_point_with_whitespace() {
        let input = "  48	,    45  ";
        assert_eq!(Ok(Point::new(48, 45)), input.parse::<Point>());
    }

    #[test]
    fn test_day5_parse_line() {
        let input = "48,233 -> 48, 456";
        assert_eq!(Ok(LineSegment::new(Point::new(48, 233), Point::new(48,456))),
                   input.parse::<LineSegment>());
    }

    #[test]
    fn day5_horizontal_segment_iterator() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(3,5));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(1,5)), iterator.next());
        assert_eq!(Some(Point::new(2,5)), iterator.next());
        assert_eq!(Some(Point::new(3,5)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn day5_vertical_segment_iterator() {
        let segment = LineSegment::new(Point::new(3,8), Point::new(3,10));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(3,8)), iterator.next());
        assert_eq!(Some(Point::new(3,9)), iterator.next());
        assert_eq!(Some(Point::new(3,10)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    // #[test]
    // fn test_day5_parse_half_line() {
    //     let input = "48,233 ->";
    //     assert_eq!(Ok(LineSegment::new(Point::new(48, 233), Point::new(48,456))),
    //                input.parse::<LineSegment>());
    // }

}
