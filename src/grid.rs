#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x, y
        }
    }
}

pub struct Grid {
    // Currently clones incoming data, maybe with a lifelife parameter it could reference the
    // original data, or just move it in?
    pub data: Vec<i32>, // pub for now...
    width: usize,
    height: usize
}

pub struct GridIter {
    index: usize,
    width: usize,
    number_of_points: usize,
}

impl Iterator for GridIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        self.index += 1;

        if current < self.number_of_points {
            Some(Point::new(current % self.width, current / self.width))
        } else {
            None
        }
    }
}

impl Grid {
    pub fn new(data: &Vec<i32>, width: usize, height: usize) -> Grid {
        // What should new do if the inputs are inconsistent? E.g. data doesn't have enough points
        // To make a complete number of rows, or is empty?
        Grid {
            data: data.clone(),
            width,
            height
        }
    }

    pub fn iter(self: &Grid) -> GridIter {
        GridIter {
            index: 0,
            width: self.width,
            number_of_points: self.width * self.height,
        }
    }

    // Should probably return an Option, for out of bounds points
    pub fn index(self: &Grid, point: &Point) -> usize {
        point.x + point.y * self.width
    }

    // Should probably return an Option, for out of bounds points
    pub fn value(self: &Grid, point: &Point) -> i32 {
        self.data[self.index(point)]
    }

    pub fn values(self: &Grid, points: &Vec<Point>) -> Vec<i32> {
        points.iter().map(|point| self.value(point)).collect()
    }

    pub fn neighbours(self: &Grid, point: &Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();

        if point.x > 0 {
            neighbours.push(Point::new(point.x - 1, point.y));
        }
        if point.x < self.width - 1 {
            neighbours.push(Point::new(point.x + 1, point.y));
        }
        if point.y > 0 {
            neighbours.push(Point::new(point.x, point.y - 1));
        }
        if point.y < self.height - 1 {
            neighbours.push(Point::new(point.x, point.y + 1));
        }

        neighbours
    }

    pub fn diagonal_neighbours(self: &Grid, point: &Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();

        if point.x > 0 && point.y > 0 {
            neighbours.push(Point::new(point.x - 1, point.y - 1));
        }
        if point.x > 0 && point.y < self.height - 1 {
            neighbours.push(Point::new(point.x - 1, point.y + 1));
        }
        if point.x < self.width - 1 && point.y > 0 {
            neighbours.push(Point::new(point.x + 1, point.y - 1));
        }
        if point.x < self.width - 1 && point.y < self.height - 1 {
            neighbours.push(Point::new(point.x + 1, point.y + 1));
        }

        neighbours
    }
}

mod tests {
    use super::*;

    #[test]
    fn grid_iterator() {
        let grid = Grid::new(&vec![0, 1, 2, 3], 2, 2);
        let mut i = grid.iter();
        assert_eq!(Some(Point::new(0, 0)), i.next());
        assert_eq!(Some(Point::new(1, 0)), i.next());
        assert_eq!(Some(Point::new(0, 1)), i.next());
        assert_eq!(Some(Point::new(1, 1)), i.next());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
    }
}
