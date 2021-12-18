use crate::grid::{Grid, Point};
use std::collections::HashSet;
use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day11-input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();
    let num_chars = lines[0].len();
    let mut data: Vec<i32> = Vec::new();
    for line in lines {
        for c in line.chars() {
            data.push((c.to_string()).parse::<i32>().unwrap());
        }
    }
    let mut octopuses: Grid = Grid::new(&data, num_chars, num_lines);
    let mut num_flashes: usize = 0;
    let num_steps = 400;
    for step in 0..num_steps {
        let flashes_this_step = octopus_step(&mut octopuses);
        num_flashes += flashes_this_step;
        if flashes_this_step == num_lines * num_chars {
            println!("All octopuses flashed at the same time on step {}", step + 1);
        }
    }
    println!("{}", octopuses);
    println!("Number of flashes after {} steps: {}", num_steps, num_flashes);
}

fn octopus_step(octopuses: &mut Grid) -> usize {
    // increase energy
    octopuses.data.iter_mut().for_each(|x| *x = *x + 1);

    // flash
    // track points which have flashed
    let mut flashes = HashSet::new();

    // track octopuses that get flashed, each point may be in here multiple times if
    // multiple neighbours flash it.
    let mut neighbours = Vec::new();

    // I tried this but I don't understand enough about how to annotate mutable borrows
    // within closures, and I get errors about both mutable and immutable borrows.
    // let mut handle_flash = |point: &Point| {
    //     flashes.insert(point.clone());
    //     let mut new_neighbours = octopuses.neighbours(&point);
    //     let mut new_diagonal_neighbours = octopuses.diagonal_neighbours(&point);
    //     for n in new_neighbours.iter_mut().chain(new_diagonal_neighbours.iter_mut()) {
    //         neighbours.push(n.clone());
    //     }
    // };

    for point in octopuses.iter_mut() {
        let energy = octopuses.value(&point);
        if energy > 9 && !flashes.contains(&point) {
            flashes.insert(point.clone());
            let mut new_neighbours = octopuses.neighbours(&point);
            let mut new_diagonal_neighbours = octopuses.diagonal_neighbours(&point);
            for n in new_neighbours.iter_mut().chain(new_diagonal_neighbours.iter_mut()) {
                neighbours.push(n.clone());
            }
        }
    }

    // while any octos that have not flashed have energy level > 9
    // increment energy of all neighbours
    // add to flash list
    while neighbours.len() > 0 {
        let mut neighbours2 = Vec::new();
        for point in neighbours.iter_mut() {
            // hmm this aspect of grid could be improved
            let index = octopuses.index(&point);
            octopuses.data[index] += 1;
            let energy = octopuses.value(&point);
            if energy > 9 && !flashes.contains(&point) {
                flashes.insert(point.clone());
                let mut new_neighbours = octopuses.neighbours(&point);
                let mut new_diagonal_neighbours = octopuses.diagonal_neighbours(&point);
                for n in new_neighbours.iter_mut().chain(new_diagonal_neighbours.iter_mut()) {
                    neighbours2.push(n.clone());
                }
            }
        }
        //println!("neighbours2: {:?}", neighbours2);
        neighbours.clear();
        neighbours.append(&mut neighbours2);
    }

    // reset flashed octopuses to 0 energy
    for flashed_octopus in &flashes {
        let index = octopuses.index(&flashed_octopus);
        octopuses.data[index] = 0;
    }

    flashes.len()
}

mod tests {
    use super::*;

    #[test]
    fn day11_octopus_increment() {
        let data: Vec<i32> = vec![1; 25];
        let mut octopuses: Grid = Grid::new(&data, 5, 5);
        octopus_step(&mut octopuses);
        assert_eq!(vec![2; 25], octopuses.data);
    }

    #[test]
    fn day11_octopus_flash() {
        let data: Vec<i32> = vec![1; 25];
        let mut octopuses: Grid = Grid::new(&data, 5, 5);

        // Interesting, this doesn't work due to borrowing both mut and immumt:
        //octopuses.data[octopuses.index(&Point::new(2, 2))] = 9;
        // But this does:
        let index = octopuses.index(&Point::new(2, 2));
        octopuses.data[index] = 9;
        octopus_step(&mut octopuses);

        let mut expected = vec![2; 25];
        expected[index] = 0;
        expected[octopuses.index(&Point::new(1,1))] = 3;
        expected[octopuses.index(&Point::new(2,1))] = 3;
        expected[octopuses.index(&Point::new(3,1))] = 3;
        expected[octopuses.index(&Point::new(1,2))] = 3;
        expected[octopuses.index(&Point::new(3,2))] = 3;
        expected[octopuses.index(&Point::new(1,3))] = 3;
        expected[octopuses.index(&Point::new(2,3))] = 3;
        expected[octopuses.index(&Point::new(3,3))] = 3;
        assert_eq!(expected, octopuses.data);
    }

    #[test]
    fn day11_octopus_flash_two_step() {
        let data: Vec<i32> = vec![1; 25];
        let mut octopuses: Grid = Grid::new(&data, 5, 5);

        // Interesting, this doesn't work due to borrowing both mut and immumt:
        //octopuses.data[octopuses.index(&Point::new(2, 2))] = 9;
        // But this does:
        let index = octopuses.index(&Point::new(2, 2));
        octopuses.data[index] = 9;
        octopus_step(&mut octopuses);
        octopus_step(&mut octopuses);
        let mut expected = vec![2; 25];
        expected[index] = 1;
        assert_eq!(expected, octopuses.data);
    }

    #[test]
    fn day11_octopus_large_example() {
        let width = 10;
        let height = 10;
        let data: Vec<i32> = vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3,
                                  2, 7, 4, 5, 8, 5, 4, 7, 1, 1,
                                  5, 2, 6, 4, 5, 5, 6, 1, 7, 3,
                                  6, 1, 4, 1, 3, 3, 6, 1, 4, 6,
                                  6, 3, 5, 7, 3, 8, 5, 4, 7, 8,
                                  4, 1, 6, 7, 5, 2, 4, 6, 4, 5,
                                  2, 1, 7, 6, 8, 4, 1, 7, 2, 1,
                                  6, 8, 8, 2, 8, 8, 1, 1, 3, 4,
                                  4, 8, 4, 6, 8, 4, 8, 5, 5, 4,
                                  5, 2, 8, 3, 7, 5, 1, 5, 2, 6];
        let mut octopuses: Grid = Grid::new(&data, width, height);

        let expected: Grid = Grid::new(&vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4,
                                             3, 8, 5, 6, 9, 6, 5, 8, 2, 2,
                                             6, 3, 7, 5, 6, 6, 7, 2, 8, 4,
                                             7, 2, 5, 2, 4, 4, 7, 2, 5, 7,
                                             7, 4, 6, 8, 4, 9, 6, 5, 8, 9,
                                             5, 2, 7, 8, 6, 3, 5, 7, 5, 6,
                                             3, 2, 8, 7, 9, 5, 2, 8, 3, 2,
                                             7, 9, 9, 3, 9, 9, 2, 2, 4, 5,
                                             5, 9, 5, 7, 9, 5, 9, 6, 6, 5,
                                             6, 3, 9, 4, 8, 6, 2, 6, 3, 7], width, height);
        octopus_step(&mut octopuses);
        assert_eq!(expected, octopuses);
        let expected: Grid = Grid::new(&vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5,
                                             5, 0, 8, 9, 0, 8, 7, 0, 5, 4,
                                             8, 5, 9, 7, 8, 8, 9, 6, 0, 8,
                                             8, 4, 8, 5, 7, 6, 9, 6, 0, 0,
                                             8, 7, 0, 0, 9, 0, 8, 8, 0, 0,
                                             6, 6, 0, 0, 0, 8, 8, 9, 8, 9,
                                             6, 8, 0, 0, 0, 0, 5, 9, 4, 3,
                                             0, 0, 0, 0, 0, 0, 7, 4, 5, 6,
                                             9, 0, 0, 0, 0, 0, 0, 8, 7, 6,
                                             8, 7, 0, 0, 0, 0, 6, 8, 4, 8], width, height);
        octopus_step(&mut octopuses);
        println!("{}", expected);
        println!("{}", octopuses);
        assert_eq!(expected, octopuses);
    }
}
