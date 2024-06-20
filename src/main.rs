mod ast;
mod expr;
mod parser;

use ast::RegexAST;
use expr::match_expr;
use std::io::{self, Write};

use parser::parse_regex;

fn main() {
    let mut pattern = String::new();
    let mut text = String::new();

    print!("Enter the regex pattern: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pattern).unwrap();
    pattern = pattern.trim().to_string();

    print!("Enter the text to match: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();
    text = text.trim().to_string();

    let parsed_pattern = match parse_regex(&pattern) {
        Ok(p) => p,
        Err(e) => {
            println!("Error parsing pattern: {}", e);
            return;
        }
    };

    let parsed_pattern_refs: Vec<&RegexAST> = parsed_pattern.iter().collect();

    match match_expr(parsed_pattern_refs, &text) {
        Ok(matches) => {
            for (start, end) in matches {
                if start >= 1 && end - start > 1 {
                    println!("Match found from index {} to {}", start, end - 1);
                } else {
                    println!("Match found from index {} to {}", start, end - 1);
                }
            }
        }
        Err(e) => println!("No match found: {}", e),
    }
}
