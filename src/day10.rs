use std::fs;

pub fn puzzle() {
    let input = fs::read_to_string("data/day10-input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let results: Vec<ParseResult> = lines.iter().map(|x| parse_input(x)).collect();
    println!("results: {:?}", results);
    let scores: Vec<i32> = results.iter().map(score_syntax_error).collect();
    println!("scores: {:?}", scores);
    let total_score: i32 = scores.iter().sum();
    println!("total score: {}", total_score);

    // part 2
    let mut completion_scores: Vec<i64> = Vec::new();
    for line in lines {
        // using a for loop because I need to keep the original output together with the Incomplete
        // result
        let result = parse_input(line);
        if result == Err(ParseError::Incomplete) {
            let completion = complete_line(line);
            let score = score_completion(&completion);
            completion_scores.push(score);
        }
    }

    completion_scores.sort();
    println!("completion scores: {:?}", completion_scores);
    let middle_score = completion_scores[(completion_scores.len() - 1) / 2];
    println!("middle completion score: {}", middle_score);
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

fn complete_line(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let opening_tokens = vec!['(', '[', '{', '<'];
    let closing_tokens = vec![')', ']', '}', '>'];

    for c in input.chars() {
        if opening_tokens.contains(&c) {
            stack.push(c);
        } else if closing_tokens.contains(&c) {
            stack.pop();
        }
    }

    stack.reverse();
    let mut output: String = String::new();
    for open in stack {
        let close = match open {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => ' ',
        };
        output += &close.to_string();
    }
    output
}

fn score_completion(completion: &str) -> i64 {
    let mut score = 0;
    for c in completion.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }
    score
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

    #[test]
    fn day10_complete_line() {
        assert_eq!("}}]])})]", complete_line("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(")}>]})", complete_line("[(()[<>])]({[<{<<[]>>("));
        assert_eq!("}}>}>))))", complete_line("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!("]]}}]}]}>", complete_line("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!("])}>", complete_line("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn day10_score_completion() {
        assert_eq!(294, score_completion("])}>"));
        assert_eq!(288957, score_completion("}}]])})]"));
        assert_eq!(5566, score_completion(")}>]})"));
        assert_eq!(1480781, score_completion("}}>}>))))"));
        assert_eq!(995444, score_completion("]]}}]}]}>"));
    }
}
