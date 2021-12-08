use std::fs;

pub fn puzzle() {
    let input: String = fs::read_to_string("data/day7-input.txt").unwrap();
    let crab_positions: Vec<i32> = input.split(',')
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect();
    let cheapest_position = find_cheapest_position(&crab_positions);
    let fuel_used = align_to_position(cheapest_position, &crab_positions);
    println!("Cheapest position to align the crabs to is: {}", cheapest_position);
    println!("Fuel cost to align to that position is: {}", fuel_used);
}

fn align_to_position(position: i32, crab_positions: &Vec<i32>) -> i32 {
    let mut fuel_used = 0;
    for crab_position in crab_positions {
        fuel_used += puzzle_2_fuel_cost(position, *crab_position);
    }
    fuel_used
}

fn puzzle_1_fuel_cost(starting_position: i32, goal_position: i32) -> i32 {
    (goal_position - starting_position).abs()
}

fn find_cheapest_position(crab_positions: &Vec<i32>) -> i32 {
    // Brute force...
    let min_position: i32 = *crab_positions.iter().min().unwrap();
    let max_position: i32 = *crab_positions.iter().max().unwrap();
    let mut best_position = min_position;
    let mut best_position_cost = align_to_position(best_position, &crab_positions);
    for position in (min_position+1)..max_position {
        let cost = align_to_position(position, crab_positions);
        if cost < best_position_cost {
            best_position = position;
            best_position_cost = cost;
        }
    }
    best_position
}

fn puzzle_2_fuel_cost(starting_position: i32, goal_position: i32) -> i32 {
    let distance = (goal_position - starting_position).abs();
    // ugh there must be a simple closed form solution to this
    let fuel_used: i32 = (1..(distance + 1)).sum();
    fuel_used
}

mod tests {
    use super::*;

    #[test]
    fn day7_crabs_align_to_position_2() {
        let crab_positions: Vec<i32> = vec!(16,1,2,0,4,2,7,1,2,14);
        let total_fuel_used = align_to_position(2, &crab_positions);
        assert_eq!(37, total_fuel_used);
    }

    #[test]
    fn day7_crabs_align_to_position_1() {
        let crab_positions: Vec<i32> = vec!(16,1,2,0,4,2,7,1,2,14);
        let total_fuel_used = align_to_position(1, &crab_positions);
        assert_eq!(41, total_fuel_used);
    }

    #[test]
    fn day7_find_cheapest_position() {
        let crab_positions: Vec<i32> = vec!(16,1,2,0,4,2,7,1,2,14);
        let cheapest_position = find_cheapest_position(&crab_positions);
        assert_eq!(2, cheapest_position);
    }

    #[test]
    fn day7_puzzle2_align_to_position() {
        let starting_position = 16;
        let goal_position = 5;
        assert_eq!(66, puzzle_2_fuel_cost(starting_position, goal_position));
    }
}