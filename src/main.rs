use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::Chars;

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
                    Err(format!("Syntax error {} is not a number", *n).to_string());
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
                let num_range: Vec<u8> = (0..=9).collect();
                if !num_range.contains(&first_char_as_digit) {
                    return None;
                }
                current_text = &current_text[1..];
                current_pos += 1;
            }
            RegexAST::OneOrMany(one_or_many) => {
                // TODO: Tenk litt mer på den her, er dette
                // måten å gjøre det på ???????
                if let Some((_, end)) = match_from_index(
                    vec![Box::into_inner(one_or_many)],
                    current_text,
                    current_pos,
                ) {
                    current_text = &current_text[(end - current_pos)..];
                    current_pos = end;
                } else {
                    return None;
                }
            }
            RegexAST::ZeroOrMany(zero_or_many) => {}
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
    let num_range: HashSet<u32> = custom_sequence(1, 9);
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
