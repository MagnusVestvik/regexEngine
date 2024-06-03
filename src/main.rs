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
    CharLiteral(char),
    NumLiteral(u8),
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

fn parse_regex(text_match: &str) -> Result<RegexAST, String> {
    let mut chars = text_match.chars().peekable();
    let mut sequence = Vec::new();
    while let Some(&c) = chars.peek() {
        match c {
            '\\' => {
                chars.next();
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'w' => sequence.push(RegexAST::Word(Box::new(RegexAST::CharLiteral('w')))),
                        's' => sequence.push(RegexAST::WhiteSpace),
                        'd' => sequence.push(RegexAST::NumLiteral(escaped as u8)),
                        _ => sequence.push(RegexAST::CharLiteral(escaped)),
                    };
                }
            }
            '.' => {
                chars.next();
                sequence.push(RegexAST::Any); // this needs to be recursivly called or something
            }
            '*' => {
                chars.next();
                if let Some(last) = sequence.pop() {
                    sequence.push(RegexAST::ZeroOrMany(Box::new(last)));
                } else {
                    return Err("Syntax error: '*' found without preceding element.".to_string());
                }
            }
            '+' => {
                chars.next();
                if let Some(last) = sequence.pop() {
                    sequence.push(RegexAST::OneOrMany(Box::new(last)));
                } else {
                    return Err("Syntax error: '+' found without preceding element.".to_string());
                }
            }
            _ => {
                chars.next();
                sequence.push(RegexAST::CharLiteral(c)); // this also needs to be recursivly called or something
            }
        };
    }
    return Ok(RegexAST::Sequence(sequence));
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
        let expected = RegexAST::Sequence(vec![RegexAST::CharLiteral('a')]);
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_any() {
        let pattern = ".";
        let expected = RegexAST::Sequence(vec![RegexAST::Any]);
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_zero_or_many() {
        let pattern = "a*";
        let expected = RegexAST::Sequence(vec![RegexAST::ZeroOrMany(Box::new(
            RegexAST::CharLiteral('a'),
        ))]);
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_one_or_many() {
        let pattern = "a+";
        let expected = RegexAST::Sequence(vec![RegexAST::OneOrMany(Box::new(
            RegexAST::CharLiteral('a'),
        ))]);
        assert_eq!(parse_regex(pattern), Ok(expected));
    }

    #[test]
    fn test_parse_combined() {
        let pattern = "a.*b+c\\d";
        let expected = RegexAST::Sequence(vec![
            RegexAST::CharLiteral('a'),
            RegexAST::ZeroOrMany(Box::new(RegexAST::Any)),
            RegexAST::CharLiteral('b'),
            RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('c'))),
            RegexAST::Word(Box::new(RegexAST::CharLiteral('d'))),
        ]);
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
