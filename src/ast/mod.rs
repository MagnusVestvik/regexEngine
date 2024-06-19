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
