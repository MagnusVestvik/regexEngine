use std::collections::HashSet;

use crate::RegexAST;
use lazy_static::lazy_static;

lazy_static! {
    static ref WORD: HashSet<char> = num_sequence_to_char(all_letters()); // TODO: add hyphen all
    // numbers, and peiod.
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

fn custom_sequence(start: u8, end: u8) -> HashSet<u8> {
    return (start..=end).collect();
}

fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn remove_subsets(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut result: Vec<(usize, usize)> = Vec::new();

    for range in ranges {
        if let Some(last) = result.last() {
            if range.0 >= last.0 && range.1 <= last.1 {
                continue;
            }
        }
        result.push(range);
    }

    result
}

fn num_sequence_to_char(range: HashSet<u32>) -> HashSet<char> {
    range
        .clone()
        .iter()
        .filter_map(|x| char::from_u32(*x))
        .collect()
}

pub fn match_expr(
    regex_expr: Vec<&RegexAST>,
    text_match: &str,
) -> Result<Vec<(usize, usize)>, String> {
    let mut matches = Vec::new();
    for start in 0..text_match.chars().count() {
        if let Some((_, end)) = match_from_index(regex_expr.clone(), &text_match[start..], start) {
            matches.push((start, end));
        }
    }
    if matches.is_empty() {
        return Err("No match found".to_string());
    }
    return Ok(remove_subsets(matches));
}

fn match_from_index(
    regex_expr: Vec<&RegexAST>,
    text: &str,
    start: usize,
) -> Option<(usize, usize)> {
    let mut current_text = text;
    let mut current_pos = start;
    if text.len() < 1 {
        return None;
    }
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
                let first_char = get_first_char(&current_text)?;
                if !first_char.is_ascii_digit() {
                    return None;
                }
                current_text = &current_text[1..];
                current_pos += 1;
            }
            RegexAST::OneOrMany(one_or_many) => {
                let mut at_least_one = false;
                while let Some((_, end)) =
                    match_from_index(vec![one_or_many.as_ref()], current_text, current_pos)
                {
                    current_text = &current_text[(end - current_pos)..];
                    current_pos = end;
                    at_least_one = true;
                }
                if !at_least_one {
                    return None;
                }
            }
            RegexAST::ZeroOrMany(zero_or_many) => {
                while let Some((_, end)) =
                    match_from_index(vec![zero_or_many.as_ref()], current_text, current_pos)
                {
                    current_text = &current_text[(end - current_pos)..];
                    current_pos = end;
                }
            }
        }
    }

    Some((start, current_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let regex = vec![&RegexAST::AnyWord]; // TODO: skal ikke word ogs√• kunne v√¶re tall ?
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

    #[test]
    fn test_match_expr_zero_or_many() {
        let zero_or_many_ast = RegexAST::ZeroOrMany(Box::new(RegexAST::CharLiteral('a')));
        let regex = vec![&zero_or_many_ast];
        let text = "aaaabaaa";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 4), (5, 8)]);
    }

    #[test]
    fn test_match_expr_zero_or_many_empty() {
        let zero_or_many_ast = RegexAST::ZeroOrMany(Box::new(RegexAST::CharLiteral('a')));
        let regex = vec![&zero_or_many_ast];
        let text = "bbbbb";
        let result = match_expr(regex, text);
        assert!(result.is_err()); // TODO: failer test
    }

    #[test]
    fn test_match_expr_zero_or_many_with_any() {
        let zero_or_many_ast = RegexAST::ZeroOrMany(Box::new(RegexAST::Any));
        let regex = vec![&zero_or_many_ast];
        let text = "abc";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 3)]);
    }

    #[test]
    fn test_match_expr_one_or_many() {
        let one_or_many_ast = RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('a')));
        let regex = vec![&one_or_many_ast];
        let text = "aaaabaaa";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 4), (5, 8)]);
    }

    #[test]
    fn test_match_expr_one_or_many_no_match() {
        let one_or_many_ast = RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('a')));
        let regex = vec![&one_or_many_ast];
        let text = "bbbbb";
        let result = match_expr(regex, text);
        assert!(result.is_err());
    }

    #[test]
    fn test_match_expr_one_or_many_with_any() {
        let one_or_many_ast = RegexAST::OneOrMany(Box::new(RegexAST::Any));
        let regex = vec![&one_or_many_ast];
        let text = "abc";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 3)]);
    }

    #[test]
    fn test_match_expr_zero_or_many_combined() {
        let zero_or_many_ast = RegexAST::ZeroOrMany(Box::new(RegexAST::CharLiteral('b')));
        let regex = vec![
            &RegexAST::CharLiteral('a'),
            &zero_or_many_ast,
            &RegexAST::CharLiteral('c'),
        ];
        let text = "abbbc abc ac";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 5), (6, 9), (10, 12)]);
    }

    #[test]
    fn test_match_expr_one_or_many_combined() {
        let one_or_many_ast = RegexAST::OneOrMany(Box::new(RegexAST::CharLiteral('b')));
        let regex = vec![
            &RegexAST::CharLiteral('a'),
            &one_or_many_ast,
            &RegexAST::CharLiteral('c'),
        ];
        let text = "abbbc abc ac";
        let result = match_expr(regex, text).unwrap();
        assert_eq!(result, vec![(0, 5), (6, 9)]);
    }
}
