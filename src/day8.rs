use std::fs;
use std::collections::HashMap;

pub fn puzzle() {
    let input = fs::read_to_string("data/day8-input.txt").unwrap();
    let entries: Vec<Entry> = input.lines().map(parse_line).collect();
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    let appearances = entries.iter().fold(0, |a, x| {
        a + count_unique_segments_digits(&x.outputs)
    });
    println!("The digits 1, 4, 7, or 8 appear in the outputs {} times", appearances);
}

fn parse_line<'a>(line: &'a str) -> Entry<'a> {
    let delimited_parts: Vec<&str> = line.split('|').collect();

    // Guard clause? Maybe there's a better way to do this in Rust.
    // I like early return for not causing a bunch of nesting, but
    // it's not necessarially good from a structured programming
    // pov, especially since `if` is an expression in Rust.
    if delimited_parts.len() != 2 {
        // Or Result<Entry, String> with an error?
        return Entry {
            patterns: vec![],
            outputs: vec![]
        }
    } else {
        let remove_whitespace = |x: &'a str| -> Option<&str> {
            let y = x.trim();
            if y.len() > 0 {
                Some(y)
            } else {
                None
            }
        };

        return Entry {
            patterns: delimited_parts[0].split(' ').filter_map(remove_whitespace).collect(),
            outputs: delimited_parts[1].split(' ').filter_map(remove_whitespace).collect()
        }
    }
}

fn count_unique_segments_digits(digits: &Vec<&str>) -> i32 {
    digits.iter().fold(0, |a, x| {
        let l = x.len();
        if l == 2 || l == 4 || l == 3 || l == 7 {
            a + 1
        } else {
            a
        }
    })
}

/**
 * Return a vector which maps a wire number 0-6 to a segment letter a-g.
 *
 * Or do I actually want a mapping of entry items to numbers?
 */
fn find_mapping(entry: &Entry) -> Vec<char> {
    let mut result = vec!['x'; 7];

    // Each wire could be connected to any of the segments.
    // We have to go through the possibilities and narrow them down.
    let mut candidates = vec!["abcdefg".to_string(); 7];
    let mut items_to_digits: HashMap<String, i32> = HashMap::new();
    for item in entry.patterns.iter().chain(entry.outputs.iter()) {
        narrow_candidates(&mut candidates, *item);
    }

    result
}

/**
 * Given a list of candidates for each wire mapping, narrows down possible candidates.
 */
fn narrow_candidates(candidates: &mut Vec<String>, item: &str) {
    let mut keep_for_wires: Vec<i32> = Vec::new();
    match item.len() {
        2 => {
            // Digit 1, keep the characters in the item for wires 2 and 5 only.
            candidates[0].retain(|segment| !item.contains(segment));
            candidates[1].retain(|segment| !item.contains(segment));
            candidates[2].retain(|segment| item.contains(segment));
            candidates[3].retain(|segment| !item.contains(segment));
            candidates[4].retain(|segment| !item.contains(segment));
            candidates[5].retain(|segment| item.contains(segment));
            candidates[6].retain(|segment| !item.contains(segment));
        },
        3 => {
            // Digit 7
            candidates[0].retain(|segment| item.contains(segment));
            candidates[1].retain(|segment| !item.contains(segment));
            candidates[2].retain(|segment| item.contains(segment));
            candidates[3].retain(|segment| !item.contains(segment));
            candidates[4].retain(|segment| !item.contains(segment));
            candidates[5].retain(|segment| item.contains(segment));
            candidates[6].retain(|segment| !item.contains(segment));
        },
        4 => {
            // Digit 4
            candidates[0].retain(|segment| !item.contains(segment));
            candidates[1].retain(|segment| item.contains(segment));
            candidates[2].retain(|segment| item.contains(segment));
            candidates[3].retain(|segment| item.contains(segment));
            candidates[4].retain(|segment| !item.contains(segment));
            candidates[5].retain(|segment| item.contains(segment));
            candidates[6].retain(|segment| !item.contains(segment));
        },
        7 => {
            // Digit 8, keep all
            candidates[0].retain(|segment| item.contains(segment));
            candidates[1].retain(|segment| item.contains(segment));
            candidates[2].retain(|segment| item.contains(segment));
            candidates[3].retain(|segment| item.contains(segment));
            candidates[4].retain(|segment| item.contains(segment));
            candidates[5].retain(|segment| item.contains(segment));
            candidates[6].retain(|segment| item.contains(segment));
        }
        _ => {}
    };
}

// The lifetime parameter 'a says that the &str references in the vectors
// must live at least as long as the Entry.
#[derive(Debug, PartialEq)]
struct Entry<'a> {
    patterns: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

mod tests {
    use super::*;

    // Normal connections:
    //  aaa     000
    // b   c   1   2
    // b   c   1   2
    //  ddd     333
    // e   f   4   5
    // e   f   4   5
    //  ggg     666
    //
    // Number the wires 0-6, and the segments a-g, so we need to find the
    // wire -> segment mapping for the scrambled display. The normal
    // mapping is:
    // let mapping = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    //
    // Digits 1, 4, 7, and 8 each have a unique number of segments active:
    // 1: 2 segments
    // 4: 4 segments
    // 7: 3 segments
    // 8: 7 segments

    #[test]
    fn day8_test_line_parsing_to_entry() {
        // Sample entry:
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let entry = Entry {
            patterns: vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"],
            outputs: vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"],
        };
        assert_eq!(entry, parse_line(line));
    }

    #[test]
    fn day8_test_counting_unique_digits() {
        let line = "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let entry = parse_line(line);
        assert_eq!(2, count_unique_segments_digits(&entry.outputs));
    }

    #[test]
    fn day8_test_narrow_candidates_digit_one() {
        let mut candidates = vec!["abcdefg".to_string(); 7];
        narrow_candidates(&mut candidates, "ab");
        assert_eq!(vec!["cdefg", "cdefg", "ab", "cdefg", "cdefg", "ab", "cdefg"], candidates);
    }

    #[test]
    fn day8_test_narrow_candidates_digit_one_and_seven() {
        let mut candidates = vec!["abcdefg".to_string(); 7];
        narrow_candidates(&mut candidates, "ab");
        narrow_candidates(&mut candidates, "dab");
        assert_eq!(vec!["d", "cefg", "ab", "cefg", "cefg", "ab", "cefg"], candidates);
    }

    #[test]
    fn day8_test_narrow_candidates_digit_one_and_seven_and_four() {
        let mut candidates = vec!["abcdefg".to_string(); 7];
        narrow_candidates(&mut candidates, "ab");
        narrow_candidates(&mut candidates, "dab");
        narrow_candidates(&mut candidates, "eafb");
        assert_eq!(vec!["d", "ef", "ab", "ef", "cg", "ab", "cg"], candidates);
    }

    #[test]
    fn day8_test_narrow_candidates_digit_one_and_seven_and_four_and_eight() {
        let mut candidates = vec!["abcdefg".to_string(); 7];
        narrow_candidates(&mut candidates, "ab");
        narrow_candidates(&mut candidates, "dab");
        narrow_candidates(&mut candidates, "eafb");
        narrow_candidates(&mut candidates, "acedgfb");
        assert_eq!(vec!["d", "ef", "ab", "ef", "cg", "ab", "cg"], candidates);
    }

    #[test]
    fn day8_test_segment_wire_mapping() {
        //  dddd
        // e    a
        // e    a
        //  ffff
        // g    b
        // g    b
        //  cccc

        // A vector who's index is the wire index, and value is the segment index
        let expected_mapping = vec!['d', 'e', 'a', 'f', 'g', 'b', 'c'];

        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let entry = parse_line(line);
        assert_eq!(expected_mapping, find_mapping(&entry));
    }

    #[test]
    fn day8_how_do_vec_and_str_work() {
        let mut candidates = vec!["abcdefg".to_string(); 7];
        let item = "ab";
        candidates[0].retain(|segment| item.contains(segment));
        assert_eq!("ab", candidates[0]);

        candidates[1].retain(|segment| !item.contains(segment));
        assert_eq!("cdefg", candidates[1]);
    }
}
