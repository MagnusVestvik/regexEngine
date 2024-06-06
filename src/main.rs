use lazy_static::lazy_static;
use std::collections::HashSet;

//////// CONSTANTS ////////
lazy_static! {
    static ref WORD: HashSet<char> = num_sequence_to_char(all_letters()); // TODO: add hyphen all
    // numbers, and peiod.
}
///// CONSTANTS ////////
//////// AST ////////
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum RegexAST {
    CharLiteral(char),
    NumLiteral(u8),
    Any,
    ZeroOrMany(Vec<RegexAST>),
    OneOrMany(Vec<RegexAST>),
    WhiteSpace,
    AnyDigit,
    AnyWord,
    Zero,
}

//////// AST ////////
//////// Semantics ////////
fn num_sequence_to_char(range: HashSet<u32>) -> HashSet<char> {
    range
        .clone()
        .iter()
        .filter_map(|x| char::from_u32(*x))
        .collect()
}

fn custom_sequence(start: u8, end: u8) -> HashSet<u8> {
    return (start..=end).collect();
}

#[allow(dead_code)]
fn whitespace(char: char) -> bool {
    if char == ' ' {
        return true;
    }
    return false;
}

fn capital_letters() -> HashSet<u32> {
    (65..=90).collect()
}

fn small_letters() -> HashSet<u32> {
    (97..=122).collect()
}

fn all_letters() -> HashSet<u32> {
    small_letters().union(&capital_letters()).copied().collect()
}

fn match_expr(regex_expr: Vec<&RegexAST>, text_match: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut matches = Vec::new();
    for start in 0..text_match.chars().count() {
        if let Some((_, end)) = match_from_index(regex_expr.clone(), &text_match[start..], start) {
            matches.push((start, end));
        }
    }
    if matches.is_empty() {
        return Err("No match found".to_string());
    }
    return Ok(matches);
}

fn match_from_index(
    regex_expr: Vec<&RegexAST>,
    text: &str,
    start: usize,
) -> Option<(usize, usize)> {
    let mut current_text = text;
    let mut current_pos = start;

    for expr in regex_expr {
        match expr {
            RegexAST::CharLiteral(c) => {
                if current_text.starts_with(*c) {
                    current_text = &current_text[1..];
                    current_pos += 1;
                } else {
                    return None;
                }
            }
            RegexAST::NumLiteral(n) => {
                if !custom_sequence(0, 9).contains(n) {
                    return None;
                }
                if current_text.starts_with((n + b'0') as char) {
                    current_text = &current_text[1..];
                    current_pos += 1;
                } else {
                    return None;
                }
            }
            RegexAST::WhiteSpace => {
                if current_text.starts_with(' ') {
                    current_text = &current_text[1..];
                    current_pos += 1;
                } else {
                    return None;
                }
            }
            RegexAST::Any => {
                current_text = &current_text[1..];
                current_pos += 1;
            }
            RegexAST::Zero => {}
            RegexAST::AnyWord => {
                let first_char = get_first_char(&current_text)?;
                if !WORD.contains(&first_char) {
                    return None;
                }
                current_text = &current_text[1..];
                current_pos += 1;
            }
            RegexAST::AnyDigit => {
                let first_char_as_digit = get_first_char(&current_text)? as u8;
                if !first_char_as_digit.is_ascii_digit() {
                    // blir ikke true
                    return None;
                }
                current_text = &current_text[1..];
                current_pos += 1;
            }
            RegexAST::OneOrMany(one_or_many) => {
                while let Some((_, end)) = match_from_index(one_or_many, current_text, current_pos)
                {
                    current_text = &current_text[(end - current_pos)..];
                    current_pos = end;
                }
            }
            RegexAST::ZeroOrMany(zero_or_many) => {
                while let Some((_, end)) = match_from_index(zero_or_many, current_text, current_pos)
                {
                    current_text = &current_text[(end - current_pos)..];
                    current_pos = end;
                }
            }
        }
    }

    Some((start, current_pos))
}

//////// Semantics ////////

//////// Helper ////////

fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

//////// Helper ////////

//////// Parser ////////

fn parse_regex(text_match: &str) -> Result<Vec<RegexAST>, String> {
    let mut chars = text_match.chars().peekable();
    let mut sequence = Vec::new();
    let mut prev: Option<char> = None;
    while let Some(&c) = chars.peek() {
        match c {
            '\\' => {
                chars.next();
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'w' => sequence.push(RegexAST::AnyWord),
                        's' => sequence.push(RegexAST::WhiteSpace),
                        'd' => sequence.push(RegexAST::AnyDigit),
                        _ => sequence.push(RegexAST::AnyWord),
                    };
                } else {
                    sequence.push(RegexAST::CharLiteral('\\'))
                }
            }
            '.' => {
                chars.next();
                sequence.push(RegexAST::Any);
            }
            '*' => {
                chars.next();
                let mut zero_or_many: Vec<RegexAST> = Vec::new();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        zero_or_many.push(ast);
                    } else {
                        return Err("Error parsing previous character".to_string());
                    }
                } else {
                    sequence.push(RegexAST::ZeroOrMany(zero_or_many));
                }
            }
            '+' => {
                chars.next();
                let mut one_or_many: Vec<RegexAST> = Vec::new();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        one_or_many.push(ast);
                    } else {
                        return Err("Error parsing previous character".to_string());
                    }
                } else {
                    return Err("Syntax error: '+' found without preceding element.".to_string());
                }
                sequence.push(RegexAST::OneOrMany(one_or_many));
            }
            _ => {
                chars.next();
                sequence.push(RegexAST::CharLiteral(c));
            }
        };
        prev = Some(c);
    }
    Ok(sequence)
}

//////// Parser ////////

//////// Tests ////////

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
    fn test_parse_empty_zero_or_many() {
        let pattern = "*";
        let expected = vec![RegexAST::ZeroOrMany(Box::new(RegexAST::Zero))];
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_one_or_many() {
        let pattern = "a+";
        let expected = vec![RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('a')))];
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

    #[test]
    fn test_match_expr_char_literal() {
        let regex = vec![&RegexAST::CharLiteral('a')];
        let text = "abcabc";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 1), (3, 4)]);
    }

    #[test]
    fn test_match_expr_num_literal() {
        let regex = vec![&RegexAST::NumLiteral(1)];
        let text = "123123";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 1), (3, 4)]);
    }

    #[test]
    fn test_match_expr_any() {
        let regex = vec![&RegexAST::Any];
        let text = "abc";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_match_expr_whitespace() {
        let regex = vec![&RegexAST::WhiteSpace];
        let text = "a b c";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn test_match_expr_any_digit() {
        let regex = vec![&RegexAST::AnyDigit];
        let text = "a1b2c3";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(1, 2), (3, 4), (5, 6)]);
    }

    #[test]
    fn test_match_expr_any_word() {
        let regex = vec![&RegexAST::AnyWord];
        let text = "abc123";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_match_expr_no_match() {
        let regex = vec![&RegexAST::CharLiteral('x')];
        let text = "abc";
        let result = match_expr(regex, text);
        assert!(result.is_err());
    }
    #[test]
    fn test_match_expr_multi_match() {
        let regex = vec![
            &RegexAST::WhiteSpace,
            &RegexAST::CharLiteral('h'),
            &RegexAST::CharLiteral('e'),
            &RegexAST::CharLiteral('l'),
            &RegexAST::CharLiteral('l'),
            &RegexAST::CharLiteral('o'),
        ];
        let text = " hello jumpa torvaldsen hello";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 6), (23, 29)]);
    }
}

//////// Tests ////////

fn main() {
    let all_letters_as_nums = all_letters();
    let all_letters = num_sequence_to_char(all_letters_as_nums);
    println!("these are all the letters \n {:?}", all_letters.into_iter());
}
