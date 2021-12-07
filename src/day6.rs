use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day6-input.txt").unwrap();

}

fn fish_step(fishes: Vec<i32>) -> Vec<i32> {
    let mut fishes_timer = Vec::new();
    fishes_timer.push(2);
    fishes_timer
}

mod tests {
    use super::*;

//     So, suppose you have a lanternfish with an internal timer value of 3:

//     After one day, its internal timer would become 2.
//     After another day, its internal timer would become 1.
//     After another day, its internal timer would become 0.
//     After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
//     After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.

// A lanternfish that creates a new fish resets its timer to 6, not 7 (because 0 is included as a valid timer value). The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.

    #[test]
    fn day6_after_one_day_timer_would_become_2() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        fishes_timer = fish_step(fishes_timer);

        assert_eq!(2, fishes_timer[0]);
    }
}