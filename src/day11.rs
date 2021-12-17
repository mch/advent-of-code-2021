use crate::grid::{Grid, Point};
use std::collections::HashSet;

pub fn puzzle() {
}

fn octopus_step(octopuses: &mut Grid) {
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
    let mut handle_flash = |point: &Point| {
        flashes.insert(point.clone());
        let mut new_neighbours = octopuses.neighbours(&point);
        let mut new_diagonal_neighbours = octopuses.diagonal_neighbours(&point);
        for n in new_neighbours.iter_mut().chain(new_diagonal_neighbours.iter_mut()) {
            neighbours.push(n.clone());
        }
    };

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
        println!("neighbours2: {:?}", neighbours2);
        neighbours.clear();
        neighbours.append(&mut neighbours2);
    }

    // reset flashed octopuses to 0 energy
    for flashed_octopus in &flashes {
        let index = octopuses.index(&flashed_octopus);
        octopuses.data[index] = 0;
    }
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
}
