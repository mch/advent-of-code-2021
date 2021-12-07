use std::fs;

pub fn puzzle1() -> () {
    let input: String = fs::read_to_string("data/day6-input.txt").unwrap();
    let mut fish_timers: Vec<i32> = input.split(',')
        .filter_map(|x| {
            let result = x.parse::<i32>();
            match result {
                Ok(value) => Some(value),
                Err(err) => {
                    println!("Failed to parse '{}' as a number: {}", x, err);
                    None
                },
            }
        })
        .collect();

    //let mut fish_timers: Vec<i32> = vec!(3,4,3,1,2);

    let solution = 2;
    if solution == 1 {
        // Track each individual fish
        let days = 256;
        let mut old_num_fish = fish_timers.len();
        for day in 0..days {
            fish_timers = fish_step(fish_timers);
            println!("After {} days, there are {} fish, increase of {}", day, fish_timers.len(), fish_timers.len() - old_num_fish);
            old_num_fish = fish_timers.len();
        }
        println!("After {} days, there are {} fish.", days, fish_timers.len());
    } else if solution == 2 {
        // Track number of fish each day instead
        let mut fish_per_day = convert_individual_fish_to_number_per_day(&fish_timers);
        let days = 256;
        for day in 0..days {
            time_step_fishes(&mut fish_per_day);
            //println!("{:?}", fish_per_day);
        }
        let total = fish_per_day.into_iter().sum::<usize>();
        println!("After {} days there are a total of {} fish.", days, total);
    }
    
}

fn fish_step(mut fish_timers: Vec<i32>) -> Vec<i32> {
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

    #[test]
    fn day6_track_num_fish_per_day() {
        // This vector tracks the number of fish on each day, where the index of the vector is the day.
        // So if we have 5 fish, with timers of 3, 4, 3, 1, 2, 
        // Then we have fish_days of: 
        // 0, 1, 1, 2, 1
        let mut fish_days: Vec<usize> = vec![0; 9];
        fish_days[1] = 1;

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(1, 0, 0, 0, 0, 0, 0, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 0, 0, 0, 0, 0, 1, 0, 1), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 0, 0, 0, 0, 1, 0, 1, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 0, 0, 0, 1, 0, 1, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 0, 0, 1, 0, 1, 0, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 0, 1, 0, 1, 0, 0, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 1, 0, 1, 0, 0, 0, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(1, 0, 1, 0, 0, 0, 0, 0, 0), fish_days);

        time_step_fishes(&mut fish_days);
        assert_eq!(vec!(0, 1, 0, 0, 0, 0, 1, 0, 1), fish_days);
    }

    #[test]
    fn day6_track_num_fish_per_day_test2() {
        // This vector tracks the number of fish on each day, where the index of the vector is the day.
        // So if we have 5 fish, with timers of 3, 4, 3, 1, 2, 
        // Then we have fish_days of: 
        // 0, 1, 1, 2, 1
        let mut fish_days: Vec<usize> = vec![0; 9];
        fish_days[1] = 1;
        fish_days[2] = 1;
        fish_days[3] = 2;
        fish_days[4] = 1;

        let days = 18;
        for _ in 0..18 {
            time_step_fishes(&mut fish_days);
            println!("{:?}", fish_days);
        }

        // I don't understand what isn't working about this: 
        // fish_days.fold(0, |s, x| x + s)
        // or this:
        //let argh: Vec<i32> = vec!(1,2,3,4);
        //let s = argh.sum();
        let mut total = 0;
        for num_fish in fish_days {
            total += num_fish;
        }
        println!("After {} days, there is a total of {} fish", days, total);
    }

    #[test]
    fn day6_track_days_convert() {
                // So if we have 5 fish, with timers of 3, 4, 3, 1, 2, 
        // Then we have fish_days of: 
        // 0, 1, 1, 2, 1
        let mut fish_timers: Vec<i32> = vec!(3,4,3,1,2);
        assert_eq!(vec![0,1,1,2,1,0,0,0,0], convert_individual_fish_to_number_per_day(&mut fish_timers));
    }
}

fn time_step_fishes(num_fish_each_day: &mut Vec<usize>) -> () {
    let num_day_zero_fish = num_fish_each_day[0];
    for index in 0..8 {
        num_fish_each_day[index] = num_fish_each_day[index + 1];
    }
    num_fish_each_day[8] = num_day_zero_fish;
    num_fish_each_day[6] += num_day_zero_fish;
}

fn convert_individual_fish_to_number_per_day(fish_timers: &Vec<i32>) -> Vec<usize> {
    let mut fish_per_day: Vec<usize> = vec![0; 9];
    for fish_timer in fish_timers {
        fish_per_day[*fish_timer as usize] += 1;
    }
    fish_per_day
}