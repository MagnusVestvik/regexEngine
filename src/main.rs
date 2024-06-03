use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::Chars;

//////// CONSTANTS ////////
lazy_static! {
    static ref ALL_CHARS: HashSet<char> = num_sequence_to_char(all_letters());
}
///// CONSTANTS ////////
//////// AST ////////
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Rangeable {
    CharLiteral,
    NumLiteral,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Word {
    WhiteSpace,
    Range(Rangeable),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum RegexAST {
    CharLiteral(char),
    NumLiteral(u8),
    Word(Box<RegexAST>),
    Any,
    Range(Box<RegexAST>, Box<RegexAST>),
    Sequence(Vec<RegexAST>),
    NewLine,
    ZeroOrMany(Box<RegexAST>),
    OneOrMany(Box<RegexAST>),
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

fn custom_sequence(start: u32, end: u32) -> HashSet<u32> {
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

//////// Semantics ////////

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
                }
            }
            '.' => {
                chars.next();
                sequence.push(RegexAST::Any);
            }
            '*' => {
                chars.next();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        sequence.push(RegexAST::ZeroOrMany(Box::new(ast)));
                    } else {
                        sequence.push(RegexAST::ZeroOrMany(Box::new(RegexAST::Zero)));
                    }
                } else {
                    return Err("Syntax error: '*' found without preceding element.".to_string());
                }
            }
            '+' => {
                chars.next();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        sequence.push(RegexAST::OneOrMany(Box::new(ast)));
                    } else {
                        return Err("Error parsing previous character".to_string());
                    }
                } else {
                    return Err("Syntax error: '+' found without preceding element.".to_string());
                }
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
fn test_custom_sequence() {
    let char_range: HashSet<u32> = custom_sequence('a' as u32, 'd' as u32);
    let num_range: HashSet<u32> = custom_sequence(1, 10);
    let printable_char_range: HashSet<char> = num_sequence_to_char(char_range);
    println!("Number Range: {:?}", num_range);
    println!("Printable Character Range: {:?}", printable_char_range);
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
}

//////// Tests ////////

fn main() {
    test_custom_sequence();
    let all_letters_as_nums = all_letters();
    let all_letters = num_sequence_to_char(all_letters_as_nums);
    println!("these are all the letters \n {:?}", all_letters.into_iter());
}
