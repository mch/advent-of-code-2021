use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day6-input.txt").unwrap();

}

fn fish_step(fish_timers: Vec<i32>) -> Vec<i32> {
    let mut new_fish_timers = Vec::new();
    let mut number_of_new_fish = 0;
    for fish_timer in fish_timers {
        let timer = fish_timer - 1;
        if timer < 0 {
            new_fish_timers.push(6);
            number_of_new_fish += 1;
        } else {
            new_fish_timers.push(timer);    
        }
    }

    for _ in 0..number_of_new_fish {
        new_fish_timers.push(8);
    }

    new_fish_timers
}

mod tests {
    use super::*;

// So, suppose you have a lanternfish with an internal timer value of 3:
//
// [x] After one day, its internal timer would become 2.
// [x] After another day, its internal timer would become 1.
// [x] After another day, its internal timer would become 0.
// [x] After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
// [ ] After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.

// A lanternfish that creates a new fish resets its timer to 6, not 7 (because 0 is included as a valid timer value). The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.

    #[test]
    fn day6_after_one_day_timer_would_become_2() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        fishes_timer = fish_step(fishes_timer);

        assert_eq!(2, fishes_timer[0]);
    }

    #[test]
    fn day6_after_two_days_timer_becomes_1() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        for _ in 0..2 {
            fishes_timer = fish_step(fishes_timer);
        }
        
        assert_eq!(1, fishes_timer[0]);
    }

    #[test]
    fn day6_after_three_days_timer_becomes_0() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        for _ in 0..3 {
            fishes_timer = fish_step(fishes_timer);
        }

        assert_eq!(0, fishes_timer[0]);
    }

    #[test]
    fn day6_after_fours_days_timer_resets_and_new_fish_created() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        for _ in 0..4 {
            fishes_timer = fish_step(fishes_timer);
        }

        assert_eq!(2, fishes_timer.len());
        assert_eq!(6, fishes_timer[0]);
        assert_eq!(8, fishes_timer[1]);
    }

    #[test]
    fn day6_after_five_days_timers_are_5_and_7() {
        let mut fishes_timer: Vec<i32> = Vec::new();
        fishes_timer.push(3);

        for _ in 0..5 {
            fishes_timer = fish_step(fishes_timer);
        }

        assert_eq!(2, fishes_timer.len());
        assert_eq!(5, fishes_timer[0]);
        assert_eq!(7, fishes_timer[1]);
    }

    #[test]
    fn day6_full_simulation() {
        let mut fishes_timer: Vec<i32> = vec!(3,4,3,1,2);

        for _ in 0..2 {
            fishes_timer = fish_step(fishes_timer);
        }

        assert_eq!(vec!(1,2,1,6,0,8), fishes_timer);
    }
}