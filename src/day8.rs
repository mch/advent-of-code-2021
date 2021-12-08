use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day8-input.txt").unwrap();
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
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
    //  aaa
    // b   c
    // b   c
    //  ddd
    // e   f
    // e   f
    //  ggg
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
}