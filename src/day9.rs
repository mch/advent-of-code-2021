pub fn puzzle() {
}

fn low_points(heightmap: &Vec<i32>) -> Vec<i32> {
    let mut low_points = Vec::<i32>::new();

    let max_column = heightmap.len() - 1;
    for (index, point) in heightmap.iter().enumerate() {
        println!("index {}, point {}", index, point);
        let column = index;
        let mut lower_than_neighbours = true;
        if column > 0 && point >= &heightmap[index - 1] {
            lower_than_neighbours = false;
        }
        if column < max_column && point >= &heightmap[index + 1] {
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
        // no low points if there are no neighbours?
        assert_eq!(vec![0], low_points(&heightmap));
    }

    #[test]
    fn day9_find_low_point_at_start_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![1,2];
        assert_eq!(vec![1], low_points(&heightmap));
    }

    #[test]
    fn day9_find_low_point_at_end_of_line() {
        // ignoring vertical boundary conditions
        let heightmap = vec![2,1];
        assert_eq!(vec![1], low_points(&heightmap));
    }
}
