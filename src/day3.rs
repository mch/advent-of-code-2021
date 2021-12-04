use std::fs;
use std::error::Error;

pub fn puzzle1() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day3-input.txt").unwrap();

    // Input:
    // 011110011110
    // 101101001111
    // 000000010101
    // ...

    let mut zeros: [u32; 12] = [0; 12];
    let mut ones: [u32; 12] = [0; 12];
    for line in input.lines() {
        for (index, character) in line.chars().enumerate() {
            match character {
                '0' => zeros[index] = zeros[index] + 1,
                '1' => ones[index] = ones[index] + 1,
                _ => ()
            }
        }
    }

    println!("count of zero bits: {:?}", zeros);
    println!("count of one bits: {:?}", ones);

    // gamma rate is based on the most common bit
    // epsilon rate is based on the least common bit
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for index in 0..12 {
        if ones[index] > zeros[index] {
            gamma = (gamma << 1) + 1;
            epsilon = epsilon << 1;
        } else {
            gamma = gamma << 1;
            epsilon = (epsilon << 1) + 1;
        }
        println!("index: {}, ones: {}, zeros: {}, gamma: {} ({:012b}), epsilon: {} ({:012b})",
                 index, ones[index], zeros[index], gamma, gamma, epsilon, epsilon);
    }

    println!("Submarine power consumption: {}", gamma * epsilon);

    Ok(())
}

pub fn puzzle2() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("aoc-day3-input.txt").unwrap();
    let numbers = input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).collect();

    // life support rating: oxygen generator rating × CO₂ scrubber rating
    let o2_generator = oxygen_generator_rating(&numbers);
    let co2_scrubber = co2_scrubber_rating(&numbers);
    let result = o2_generator * co2_scrubber;

    println!("o2 rating: {}, co2 rating: {}, result: {}", o2_generator, co2_scrubber, result);

    Ok(())
}

fn parse_line(line: &str) -> () {
    println!("line: {}", line);
    for (index, character) in line.chars().enumerate() {
        println!("index: {}, char: {}", index, character);
    }
    ()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).collect()
}

fn count_bits(position: u32, numbers: &Vec<u32>) -> (u32, u32) {
    let mut zeros = 0;
    let mut ones = 0;
    for number in numbers {
        let bit = (number >> (position)) & 1;
        if bit == 0 {
            zeros = zeros + 1;
        } else {
            ones = ones + 1;
        }
    }
    (zeros, ones)
}

enum PreferredBit {
    MostCommon,
    LeastCommon,
}

fn filter_by_bit_position(starting_position: u32, numbers: &Vec<u32>, preferred_bit: PreferredBit) -> u32 {
    let mut position = starting_position;
    let mut nums = numbers.clone();
    while nums.len() > 1 {
        let (num_zeros, num_ones) = count_bits(position, &nums);
        let bit_to_keep = match preferred_bit {
            PreferredBit::MostCommon => if num_zeros > num_ones { 0 } else { 1 },
            PreferredBit::LeastCommon => if num_zeros <= num_ones { 0 } else { 1 },
        };
        nums = nums.into_iter().filter(|&x| x >> position & 1 == bit_to_keep).collect();
        if position == 0 { break }
        position = position - 1;
    }
    nums[0]
}

fn oxygen_generator_rating(numbers: &Vec<u32>) -> u32 {
    filter_by_bit_position(11, numbers, PreferredBit::MostCommon)
}

fn co2_scrubber_rating(numbers: &Vec<u32>) -> u32 {
    filter_by_bit_position(11, numbers, PreferredBit::LeastCommon)
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_data: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_things() {
        let n = u32::from_str_radix("10110", 2).unwrap();
        for i  in 0..5 {
            println!("{:05b}, {}, {}", n >> i, (n >> i) & 1, ((n >> i) & 1) == 0);
        }
    }

    #[test]
    fn test_count_common_bit() {
        let numbers = parse_input(test_data);
        assert_eq!((7, 5), count_bits(0, &numbers));
        assert_eq!((5, 7), count_bits(1, &numbers));
        assert_eq!((4, 8), count_bits(2, &numbers));
        assert_eq!((7, 5), count_bits(3, &numbers));
        assert_eq!((5, 7), count_bits(4, &numbers));
    }

    #[test]
    fn test_filtering() {
        let numbers = parse_input(test_data);
        assert_eq!(filter_by_bit_position(4, &numbers, PreferredBit::MostCommon), 23);
        assert_eq!(filter_by_bit_position(4, &numbers, PreferredBit::LeastCommon), 10);
    }

    #[test]
    fn test_bit_string_to_number_conversion() {
        assert_eq!(u32::from_str_radix("10110", 2), Ok(22));
    }
}

