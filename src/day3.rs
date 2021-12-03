use std::fs;
use std::error::Error;

pub fn day3() -> Result<(), Box<dyn Error>> {
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

fn parse_line(line: &str) -> () {
    println!("line: {}", line);
    for (index, character) in line.chars().enumerate() {
        println!("index: {}, char: {}", index, character);
    }
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_conversion() {
        //assert_eq!(bits("10110"), 22);

    }
}

