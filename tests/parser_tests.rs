#[cfg(test)]
mod parser;
mod regex_ast;
mod parser_tests {
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
