use crate::grid::{Grid, Point};

pub fn puzzle() {
}

fn octopus_step(octopuses: &mut Grid) {
    // increase energy
    octopuses.data.iter_mut().for_each(|x| *x = *x + 1);

    // flash
    // track points which have flashed
    let mut flashes: Vec<Point> = Vec::new();

    // I'm not sure why octopuses is being moved into the for loop or why I can't
    // use for point in &octopuses...
    for point in octopuses.iter_mut() {
        let energy = octopuses.value(&point);
        if energy > 9 {
            flashes.push(point);
        }
    }
    // while any octos that have not flashed have energy level > 9
    // increment energy of all neighbours
    // add to flash list

    // reset flashed octopuses to 0 enery
}

mod tests {
    use super::*;

    #[test]
    fn day11_octopus_flash() {
        let data: Vec<i32> = vec![1; 25];
        let mut octopuses: Grid = Grid::new(&data, 5, 5);
        octopus_step(&mut octopuses);
        assert_eq!(vec![2; 25], octopuses.data);
    }
}
