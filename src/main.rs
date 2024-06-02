use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::Chars;

/*
* bools array med true hvor det er match false ellers sjekker deretter lengden av true for å se om
* det stemmer overens med lengden av pattern.
*/
//////// CONSTANTS ////////
lazy_static! {
    static ref ALL_CHARS: HashSet<char> = num_sequence_to_char(all_letters());
}
///// CONSTANTS ////////
//////// AST ////////
#[allow(dead_code)]
enum Rangeable {
    CharLiteral(char),
    NumLiteral(char),
}

#[allow(dead_code)]
enum Word {
    WhiteSpace,
    Range(Rangeable),
}
#[allow(dead_code)]
enum RegexAST {
    Word(Word),
    Any(Box<RegexAST>),
    Range(Rangeable, Rangeable),
    Sequence(Vec<RegexAST>),
    NewLine,
    ZeroOrMany,
    OneOrMany,
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
                match chars.next() {
                    'w' => "",
                    's' => "",
                    'd' => sequence.push(Rangeable::NumLiteral(c)),
                    _ => sequence.push(Rangeable::CharLiteral(c)),
                };
            }
            '.' => {}
            '*' => {}
            '+' => {}
            _ => sequence.push(Rangeable::CharLiteral(c)),
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
//////// Tests ////////

fn main() {
    test_custom_sequence();
    let all_letters_as_nums = all_letters();
    let all_letters = num_sequence_to_char(all_letters_as_nums);
    println!("these are all the letters \n {:?}", all_letters.into_iter());
}