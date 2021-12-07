pub fn puzzle() {

}

fn align_to_position(position: i32, crab_positions: &Vec<i32>) -> i32 {
    let mut fuel_used = 0;
    for crab_position in crab_positions {
        fuel_used += (*crab_position - position).abs();
    }
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
}