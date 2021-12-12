pub fn puzzle() {

}

type ParseOutput = Vec<char>;

#[derive(Debug, PartialEq)]
enum ParseError {
    Corrupt(char),
    Incomplete,
}

type ParseResult= Result<ParseOutput, ParseError>;

fn parse_input(input: &str) -> Result<ParseOutput, ParseError> {
    let mut chunks: Vec<char> = Vec::new();
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
                return Err(ParseError::Corrupt(c))
            } else {
                chunks.push(opening_char);
            }
        }
    }

    if stack.len() > 0 {
        Err(ParseError::Incomplete)
    } else {
        Ok(chunks)
    }
}

fn score_syntax_error(result: &ParseResult) -> i32 {
    match result {
        Ok(_) => 0,
        Err(e) => {
            match e {
                ParseError::Incomplete => 0,
                ParseError::Corrupt(')') => 3,
                ParseError::Corrupt(']') => 57,
                ParseError::Corrupt('}') => 1197,
                ParseError::Corrupt('>') => 25137,
                ParseError::Corrupt(_) => 0,

            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn day10_parse_leaf_navigation_chunk() {
        assert_eq!(Ok(vec!('(')), parse_input("()"));
        assert_eq!(Ok(vec!('[')), parse_input("[]"));
        assert_eq!(Ok(vec!('{')), parse_input("{}"));
        assert_eq!(Ok(vec!('<')), parse_input("<>"));
    }

    #[test]
    fn day10_parse_corrupt_navigation_chunk() {
        assert_eq!(Err(ParseError::Corrupt(']')), parse_input("(]"));
        assert_eq!(Err(ParseError::Corrupt(')')), parse_input("[)"));
        assert_eq!(Err(ParseError::Corrupt('>')), parse_input("{>"));
        assert_eq!(Err(ParseError::Corrupt('}')), parse_input("<}"));
    }

    #[test]
    fn day10_parse_nested_chunks() {
        assert_eq!(true, parse_input("([<{}>])").is_ok());
    }

    #[test]
    fn day10_parse_nested_corrupt_chunks() {
        assert_eq!(Err(ParseError::Corrupt('}')), parse_input("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(Err(ParseError::Corrupt(')')), parse_input("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(Err(ParseError::Corrupt(']')), parse_input("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(Err(ParseError::Corrupt(')')), parse_input("[<(<(<(<{}))><([]([]()"));
        assert_eq!(Err(ParseError::Corrupt('>')), parse_input("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn day10_syntax_error_score() {
        let results = vec![
            Err(ParseError::Corrupt('}')),
            Err(ParseError::Corrupt(')')),
            Err(ParseError::Corrupt(']')),
            Err(ParseError::Corrupt(')')),
            Err(ParseError::Corrupt('>')),
        ];
        assert_eq!(26397, results.iter().map(score_syntax_error).sum());
    }

    #[test]
    fn day10_incomplete_chunk() {
        assert_eq!(Err(ParseError::Incomplete), parse_input("({}"));
    }

    #[test]
    fn day10_multiple_chunks_on_one_line() {
        assert_eq!(true, parse_input("<>()[]").is_ok());
    }
}
