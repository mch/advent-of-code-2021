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
            if c != matching_closing_char {
                return ParseResult::Corrupt(c)
            }
        }
    }
    ParseResult::Chunk(input.chars().nth(0).unwrap())
}

fn score_syntax_error(result: &ParseResult) -> i32 {
    match *result {
        ParseResult::Chunk(_) => 0,
        ParseResult::Corrupt(')') => 3,
        ParseResult::Corrupt(']') => 57,
        ParseResult::Corrupt('}') => 1197,
        ParseResult::Corrupt('>') => 25137,
        ParseResult::Corrupt(_) => 0,
    }
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

    #[test]
    fn day10_parse_nested_chunks() {
        assert_eq!(ParseResult::Chunk('('), parse_input("([<{}>])"));
    }

    #[test]
    fn day10_parse_nested_corrupt_chunks() {
        assert_eq!(ParseResult::Corrupt('}'), parse_input("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(ParseResult::Corrupt(')'), parse_input("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(ParseResult::Corrupt(']'), parse_input("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(ParseResult::Corrupt(')'), parse_input("[<(<(<(<{}))><([]([]()"));
        assert_eq!(ParseResult::Corrupt('>'), parse_input("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn day10_syntax_error_score() {
        let results = vec![
            ParseResult::Corrupt('}'),
            ParseResult::Corrupt(')'),
            ParseResult::Corrupt(']'),
            ParseResult::Corrupt(')'),
            ParseResult::Corrupt('>'),
        ];
        assert_eq!(26397, results.iter().map(score_syntax_error).sum());
    }
}
