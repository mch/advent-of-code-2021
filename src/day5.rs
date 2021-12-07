use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day5-input.txt").unwrap();
    let segments: Vec<LineSegment> = input.lines()
        .map(|line| line.parse::<LineSegment>().unwrap())
        .collect();
    //println!("Segments: {:?}", segments);

    let mut points = HashMap::new();
    for segment in &segments {
        if segment.is_axis_aligned() {
            for point in segment.iter() {
                let count = points.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    println!("Number of axis aligned points known: {:?}", points.len());
    println!("Number of axis aligned points with overlapping segments: {}",
             points.iter().filter(|(_,v)| **v >= 2).count());

    let mut points2 = HashMap::new();
    for segment in &segments {
        if segment.is_axis_aligned() || segment.is_diagonal() {
            for point in segment.iter() {
                let count = points2.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    println!("Number of axis aligned or diagonal points known: {:?}", points2.len());
    println!("Number of axis aligned or diagonal points with overlapping segments: {}",
             points2.iter().filter(|(_,v)| **v >= 2).count());
}

#[derive(Debug, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

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

    fn is_axis_aligned(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    fn is_diagonal(&self) -> bool {
        (self.b.x - self.a.x).abs() == (self.b.y - self.a.y).abs()
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
        let direction = Point::new((segment.b.x - segment.a.x).signum(),
                                   (segment.b.y - segment.a.y).signum());
        LineSegmentIter {
            segment: *segment,
            direction,
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
    fn day5_horizontal_segment_is_axis_aligned() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(3,5));
        assert_eq!(true, segment.is_axis_aligned());
    }

    #[test]
    fn day5_vertical_segment_is_axis_aligned() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(1,0));
        assert_eq!(true, segment.is_axis_aligned());
    }

    #[test]
    fn day5_diagonal_segment_is_not_axis_aligned() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(2,6));
        assert_eq!(false, segment.is_axis_aligned());
    }

    #[test]
    fn day5_45_degree_segment_is_diagonal() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(2,6));
        assert_eq!(true, segment.is_diagonal());
    }

    #[test]
    fn day5_135_degree_segment_is_diagonal() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(0,6));
        assert_eq!(true, segment.is_diagonal());
    }

    #[test]
    fn day5_225_degree_segment_is_diagonal() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(0,4));
        assert_eq!(true, segment.is_diagonal());
    }

    #[test]
    fn day5_other_angled_segment_is_not_diagonal() {
        let segment = LineSegment::new(Point::new(1,5), Point::new(10,4));
        assert_eq!(false, segment.is_diagonal());
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

    #[test]
    fn day5_diagonal_segment_iterator() {
        let segment = LineSegment::new(Point::new(10,10), Point::new(12,12));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(10,10)), iterator.next());
        assert_eq!(Some(Point::new(11,11)), iterator.next());
        assert_eq!(Some(Point::new(12,12)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn day5_diagonal2_segment_iterator() {
        let segment = LineSegment::new(Point::new(10,10), Point::new(8,8));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(10,10)), iterator.next());
        assert_eq!(Some(Point::new(9,9)), iterator.next());
        assert_eq!(Some(Point::new(8,8)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn day5_diagonal3_segment_iterator() {
        let segment = LineSegment::new(Point::new(10,10), Point::new(12,8));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(10,10)), iterator.next());
        assert_eq!(Some(Point::new(11,9)), iterator.next());
        assert_eq!(Some(Point::new(12,8)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn day5_diagonal4_segment_iterator() {
        let segment = LineSegment::new(Point::new(10,10), Point::new(8,12));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(10,10)), iterator.next());
        assert_eq!(Some(Point::new(9,11)), iterator.next());
        assert_eq!(Some(Point::new(8,12)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn day5_broken_diagonal_iterator() {
        let segment = LineSegment::new(Point::new(274, 585), Point::new(651, 962));
        let mut iterator = segment.iter();
        assert_eq!(Some(Point::new(274, 585)), iterator.next());
        assert_eq!(Some(Point::new(275, 586)), iterator.next());
    }

    // #[test]
    // fn test_day5_parse_half_line() {
    //     let input = "48,233 ->";
    //     assert_eq!(Ok(LineSegment::new(Point::new(48, 233), Point::new(48,456))),
    //                input.parse::<LineSegment>());
    // }

}
