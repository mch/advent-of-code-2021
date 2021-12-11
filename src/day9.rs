use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day9-input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let number_rows = lines.len();
    let number_columns = lines[0].len();
    println!("Heightmap size: {}x{}", number_rows, number_columns);
    // let heightmap = lines.iter()
    //     .map(|line| line.trim().chars()
    //          .map(|c| c.to_string().parse::<i32>().unwrap()).collect()).collect();
    let mut heightmap: Vec<i32> = Vec::new();
    for line in lines {
        for character in line.trim().chars() {
            heightmap.push(character.to_string().parse().unwrap());
        }
    }
    let low_points = low_points(&heightmap, number_columns);
    println!("Low points: {:?}", low_points);
    let risk_levels: Vec<i32> = low_points.iter().map(|p| p.value + 1).collect();
    let risk_level_sum: i32 = risk_levels.iter().sum();
    println!("risk levels: {:?}", risk_levels);
    println!("risk level sum: {}", risk_level_sum);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    value: i32
}

impl Point {
    fn new(x: usize, y: usize, value: i32) -> Point {
        Point {
            x, y, value
        }
    }
}

struct Heightmap {
    data: Vec<i32>,
    width: usize,
    height: usize
}

impl Heightmap {
    fn new(data: &Vec<i32>, number_columns: usize) -> Heightmap {
        // What should new do if the inputs are inconsistent? E.g. data doesn't have enough points
        // To make a complete number of rows, or is empty?
        Heightmap {
            data: data.clone(),
            width: number_columns,
            height: data.len() / number_columns
        }
    }

    // Should probably return an Option, for out of bounds points
    fn index(self: &Heightmap, point: &Point) -> usize {
        point.x + point.y * self.width
    }

    // Should probably return an Option, for out of bounds points
    fn value(self: &Heightmap, point: &Point) -> i32 {
        self.data[self.index(point)]
    }

    fn point_values(self: &Heightmap, points: &Vec<Point>) -> Vec<i32> {
        points.iter().map(|point| self.value(point)).collect()
    }

}

fn low_points(heightmap: &Vec<i32>, number_columns: usize) -> Vec<Point> {
    let mut low_points = Vec::<Point>::new();

    let max_column = number_columns - 1;
    let max_row = heightmap.len() / number_columns - 1;
    //println!("heightmap size: {}x{}", max_row + 1, max_column + 1);
    for (index, point) in heightmap.iter().enumerate() {
        let column = index % number_columns;
        let row = index / number_columns;
        //println!("index {}, position {}x{}, point {}", index, row, column, point);
        let mut lower_than_neighbours = true;

        // left
        if column > 0 && point >= &heightmap[index - 1] {
            lower_than_neighbours = false;
        }

        // right
        if column < max_column && point >= &heightmap[index + 1] {
            lower_than_neighbours = false;
        }

        // up
        if row > 0 && point >= &heightmap[index - number_columns] {
            lower_than_neighbours = false;
        }

        // down
        if row < max_row && point >= &heightmap[index + number_columns] {
            lower_than_neighbours = false;
        }

        if lower_than_neighbours {
            low_points.push(Point {
                x: row,
                y: column,
                value: *point,
            });
        }
    }

    low_points
}

fn find_basin(heightmap: &Heightmap, point: &Point) -> Vec<Point> {
    let points_in_basin: Vec<Point> = Vec::new();

    points_in_basin
}

fn basin_size(heightmap: &Vec<i32>, number_columns: usize, point: &Point) -> i32 {
    0
}

mod tests {
    use super::*;

    fn values(low_points: &Vec<Point>) -> Vec<i32> {
        low_points.iter().map(|p| p.value).collect::<Vec<i32>>()
    }

    #[test]
    fn day9_find_low_point_in_point() {
        // ignoring boundary conditions
        let heightmap = vec![0];
        let number_columns = 1;
        // no low points if there are no neighbours?
        assert_eq!(vec![0], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_at_start_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![1,2];
        let number_columns = 2;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_at_end_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![2,1];
        let number_columns = 2;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_at_start_of_column() {
        // ignoring horizontal boundary conditions
        let heightmap = vec![1,2];
        let number_columns = 1;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_at_end_of_column() {
        // ignoring horizontal boundary conditions
        let heightmap = vec![2,1];
        let number_columns = 1;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_row() {
        let heightmap = vec![2,1,2];
        let number_columns = 3;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_column() {
        let heightmap = vec![2,1,2];
        let number_columns = 1;
        assert_eq!(vec![1], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_map() {
        let heightmap = vec![4,2,3,
                             7,0,5,
                             3,2,4];
        let number_columns = 3;
        assert_eq!(vec![0], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_example() {
        let heightmap = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                             3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                             9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                             8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                             9, 8, 9, 9, 9, 6, 5, 6, 7, 8];
        let number_columns = 10;
        assert_eq!(vec![1, 0, 5, 5], values(&low_points(&heightmap, number_columns)));
    }

    #[test]
    fn day9_find_basin() {
        let heightmap_data = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                                  3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                                  9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                                  8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                                  9, 8, 9, 9, 9, 6, 5, 6, 7, 8];
        let number_columns = 10;
        let heightmap: Heightmap = Heightmap::new(&heightmap_data, number_columns);
        let points: Vec<Point> = low_points(&heightmap_data, number_columns);
        assert_eq!(vec![1, 0, 5, 5], values(&points));
        assert_eq!(vec![Point::new(0, 0, 2), Point::new(1, 0, 1), Point::new(0, 1, 3)],
                   find_basin(&heightmap, &points[0]));
    }
    #[test]
    fn day9_heightmap_point_index() {
        let heightmap_data = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                                  3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                                  9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                                  8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                                  9, 8, 9, 9, 9, 6, 5, 6, 7, 8];
        let number_columns = 10;
        let heightmap: Heightmap = Heightmap::new(&heightmap_data, number_columns);
        assert_eq!(0, heightmap.index(&Point::new(0, 0, 0)));
        assert_eq!(1, heightmap.index(&Point::new(1, 0, 0)));
        assert_eq!(10, heightmap.index(&Point::new(0, 1, 0)));
        assert_eq!(11, heightmap.index(&Point::new(1, 1, 0)));
    }

    #[test]
    fn day9_heightmap_point_value() {
        let heightmap_data = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                                  3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                                  9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                                  8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                                  9, 8, 9, 9, 9, 6, 5, 6, 7, 8];
        let number_columns = 10;
        let heightmap: Heightmap = Heightmap::new(&heightmap_data, number_columns);
        assert_eq!(2, heightmap.value(&Point::new(0, 0, 0)));
        assert_eq!(1, heightmap.value(&Point::new(1, 0, 0)));
        assert_eq!(3, heightmap.value(&Point::new(0, 1, 0)));
        assert_eq!(9, heightmap.value(&Point::new(1, 1, 0)));
    }

    #[test]
    fn day9_heightmap_point_values() {
        let heightmap_data = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                                  3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                                  9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                                  8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                                  9, 8, 9, 9, 9, 6, 5, 6, 7, 8];
        let number_columns = 10;
        let heightmap: Heightmap = Heightmap::new(&heightmap_data, number_columns);
        let points: Vec<Point> = vec![Point::new(0, 0, 0),
                                      Point::new(1, 0, 0),
                                      Point::new(0, 1, 0),
                                      Point::new(1, 1, 0)];
        assert_eq!(vec![2, 1, 3, 9], heightmap.point_values(&points));
    }
}
