use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() -> () {
    let mut solutions: Vec<Box<dyn Fn() -> ()>> = Vec::new();
    solutions.push(Box::new(|| { println!("Days of the month start with 1!"); }));
    solutions.push(Box::new(day1::day1));
    solutions.push(Box::new(|| { day2::day2(); day2::day2p2(); }));
    solutions.push(Box::new(|| { day3::puzzle1(); day3::puzzle2(); }));
    solutions.push(Box::new(day4::puzzle1_and_2));
    solutions.push(Box::new(day5::puzzle1));
    solutions.push(Box::new(day6::puzzle1));
    solutions.push(Box::new(day7::puzzle));

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No day specified, running all all days...\n");
        let mut solution_iter = solutions.iter();
        solution_iter.next(); // Skip day 0
        for (index, solution) in solution_iter.enumerate() {
            println!("Running day {} solution...", index + 1);
            solution();
            println!("");
        }
    } else {
        let day_result: Result<usize, <usize as std::str::FromStr>::Err> = args[1].parse();
        day_result.map_or_else(|e| {
            println!("'{}' is not a valid day of the month: {}", args[1], e);
        }, |day| {
            if day > solutions.len() {
                println!("Day {} hasn't been solved yet!", day);
            } else {
                solutions[day]();
            }
        })
    }
}



