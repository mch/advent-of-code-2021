pub fn puzzle() {
}

fn low_points(heightmap: &Vec<i32>, number_columns: usize) -> Vec<i32> {
    let mut low_points = Vec::<i32>::new();

    let max_column = number_columns - 1;
    let max_row = heightmap.len() / number_columns - 1;
    println!("heightmap size: {}x{}", max_row + 1, max_column + 1);
    for (index, point) in heightmap.iter().enumerate() {
        let column = index % number_columns;
        let row = index / number_columns;
        println!("index {}, position {}x{}, point {}", index, row, column, point);
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
            low_points.push(*point);
        }
    }

    low_points
}

mod tests {
    use super::*;

    #[test]
    fn day9_find_low_point_in_point() {
        // ignoring boundary conditions
        let heightmap = vec![0];
        let number_columns = 1;
        // no low points if there are no neighbours?
        assert_eq!(vec![0], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_at_start_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![1,2];
        let number_columns = 2;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_at_end_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![2,1];
        let number_columns = 2;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_at_start_of_column() {
        // ignoring horizontal boundary conditions
        let heightmap = vec![1,2];
        let number_columns = 1;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_at_end_of_column() {
        // ignoring horizontal boundary conditions
        let heightmap = vec![2,1];
        let number_columns = 1;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_row() {
        let heightmap = vec![2,1,2];
        let number_columns = 3;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_column() {
        let heightmap = vec![2,1,2];
        let number_columns = 1;
        assert_eq!(vec![1], low_points(&heightmap, number_columns));
    }

    #[test]
    fn day9_find_low_point_in_middle_of_map() {
        let heightmap = vec![4,2,3,
                             7,0,5,
                             3,2,4];
        let number_columns = 3;
        assert_eq!(vec![0], low_points(&heightmap, number_columns));
    }

}
