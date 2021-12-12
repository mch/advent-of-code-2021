pub fn puzzle() {
    
}

#[derive(Debug, PartialEq)]
enum ParseResult {
    Chunk(char),
    Corrupt(char),
}

fn parse_input(input: &str) -> ParseResult {
    let mut stack: Vec<char> = Vec::new();
    let opening_tokens = vec!['(', '[', '{', '<'];
    let closing_tokens = vec![')', ']', '}', '>'];
    for c in input.chars() {
        if opening_tokens.contains(&c) {
            stack.push(c);
        } else if closing_tokens.contains(&c) {
            let opening_char = stack.pop().unwrap();
            let index = opening_tokens.iter().position(|&item| item == opening_char).unwrap();
            let matching_closing_char = closing_tokens[index];
            if c == matching_closing_char {
                return ParseResult::Chunk(opening_char)
            } else {
                return ParseResult::Corrupt(c)
            }
        }
    }
    ParseResult::Chunk(input.chars().nth(0).unwrap())
}

mod tests {
    use super::*;

    #[test]
    fn day10_parse_leaf_navigation_chunk() {
        assert_eq!(ParseResult::Chunk('('), parse_input("()"));
        assert_eq!(ParseResult::Chunk('['), parse_input("[]"));
        assert_eq!(ParseResult::Chunk('{'), parse_input("{}"));
        assert_eq!(ParseResult::Chunk('<'), parse_input("<>"));
    }

    #[test]
    fn day10_parse_corrupt_navigation_chunk() {
        assert_eq!(ParseResult::Corrupt(']'), parse_input("(]"));
        assert_eq!(ParseResult::Corrupt(')'), parse_input("[)"));
        assert_eq!(ParseResult::Corrupt('>'), parse_input("{>"));
        assert_eq!(ParseResult::Corrupt('}'), parse_input("<}"));
    }
}
