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
enum RegexAST {
    CharLiteral(char),
    NumLiteral(u8),
    Any,
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

fn match_expr(regex_expr: Vec<&RegexAST>, text_match: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut matches = Vec::new();
    for start in 0..text_match.chars().count() {
        if let Some((_, end)) =
            match_from_index(regex_expr.clone(), &text_match[start..].chars(), start)
        {
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
    text: &Chars,
    start: usize,
) -> Option<(usize, usize)> {
    let mut current_text = text.peekable();
    let mut pos = start;

    for expr in regex_expr {
        match expr {
            RegexAST::CharLiteral(c) => {
                if let Some(current_elem) = current_text.peek() {
                    if current_elem == c {
                        pos += 1;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            RegexAST::NumLiteral(n) => {
                if let Some(current_elem) = current_text.peek() {
                    if current_elem == n as char {
                        pos += 1;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            RegexAST::WhiteSpace => {}
            RegexAST::Any => {}
            RegexAST::Zero => {}
            RegexAST::AnyWord(word) => {} // TODO: legg til implementasjon av anyword slik i parser
            RegexAST::AnyDigit(digit) => {} // TODO: legg til implementasjon av anydigit slik i parser
            RegexAST::OneOrMany(one_or_many) => {}
            RegexAST::ZeroOrMany(zero_or_many) => {}
        }
    }

    Some((start, pos))
}

//////// Semantics ////////

//////// Parser ////////

fn parse_regex(text_match: &str) -> Result<Vec<RegexAST>, String> {
    let mut chars = text_match.chars().peekable();
    let mut sequence = Vec::new();
    let mut prev: Option<char> = None;
    while let Some(&c) = chars.peek() {
        // TODO: update this to chars.next such that iterator does
        // not have to be pushed forward in the functions, see example from improvment suggestions
        // from chatgpt https://chatgpt.com/c/cee54427-e4a2-4f4c-ae46-23e2865195f6
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
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        sequence.push(RegexAST::ZeroOrMany(Box::new(ast)));
                    } else {
                        return Err("Error parsing previous character".to_string());
                    }
                } else {
                    sequence.push(RegexAST::ZeroOrMany(Box::new(RegexAST::Zero)));
                }
            }
            '+' => {
                chars.next();
                if let Some(prev_char) = prev {
                    sequence.pop();
                    let parsed = parse_regex(&prev_char.to_string())?;
                    if let Some(ast) = parsed.into_iter().next() {
                        // TODO: make parsed into a iterator
                        // and use its next method for pushing correct value instead of creating
                        // new variable
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
