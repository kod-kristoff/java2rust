#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum TokenKind {
    ErrorToken, Eof,

    LParen, RParen, LCurly, RCurly,
    Eq, Semi, Comma, Colon, Arrow,
    Plus, Minus, Star, Slash, 
    Space, NewLine,

    ClassKeyword, ReturnKeyword,
    TrueKeyword, FalseKeyword,
    MultiLineCommentStart, MultiLineCommentEnd,

    Name, JString,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
