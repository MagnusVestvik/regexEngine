use crate::RegexAST;

pub fn parse_regex(text_match: &str) -> Result<Vec<RegexAST>, String> {
    let mut chars = text_match.chars().peekable();
    let mut sequence = Vec::new();
    let mut prev: Option<char> = None;

    while let Some(&c) = chars.peek() {
        match c {
            '\\' => {
                chars.next();
                match chars.next() {
                    Some('w') => sequence.push(RegexAST::AnyWord),
                    Some('s') => sequence.push(RegexAST::WhiteSpace),
                    Some('d') => sequence.push(RegexAST::AnyDigit),
                    Some(escaped) => sequence.push(RegexAST::CharLiteral(escaped)),
                    None => sequence.push(RegexAST::CharLiteral('\\')),
                }
            }
            '.' => {
                chars.next();
                sequence.push(RegexAST::Any);
            }
            '*' | '+' => {
                let operator = chars.next().unwrap();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        let regex_ast = match operator {
                            '*' => RegexAST::ZeroOrMany(Box::new(ast)),
                            '+' => RegexAST::OneOrMany(Box::new(ast)),
                            _ => unreachable!(),
                        };
                        sequence.push(regex_ast);
                    } else {
                        return Err("Error parsing previous character".to_string());
                    }
                } else {
                    return Err(format!(
                        "Syntax error: '{}' found without preceding element.",
                        operator
                    ));
                }
            }
            _ => {
                chars.next();
                sequence.push(RegexAST::CharLiteral(c));
            }
        }
        prev = Some(c);
    }

    Ok(sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let pattern = "a";
        let expected = vec![RegexAST::CharLiteral('a')];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_any() {
        let pattern = ".";
        let expected = vec![RegexAST::Any];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_zero_or_many() {
        let pattern = "a*";
        let expected = vec![RegexAST::ZeroOrMany(Box::new(RegexAST::CharLiteral('a')))];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    #[should_panic]
    fn test_parse_empty_zero_or_many() {
        let pattern = "*";
        let expected = vec![RegexAST::ZeroOrMany(Box::new(RegexAST::Any))];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_combined() {
        let pattern = "a.*b+c\\d";
        let expected = vec![
            RegexAST::CharLiteral('a'),
            RegexAST::ZeroOrMany(Box::new(RegexAST::Any)),
            RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('b'))),
            RegexAST::CharLiteral('c'),
            RegexAST::AnyDigit,
        ];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }
}
