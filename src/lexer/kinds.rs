#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Identifier,
    Space,
    Separator,
    Newline,
    QuotedIdentifier,
    Unknown,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start_position: usize,
    pub end_position: usize,
    pub token: String,
}

impl Token {
    pub fn new(kind: TokenKind, start_position: usize, end_position: usize, token: String) -> Self {
        Token {
            kind,
            start_position,
            end_position,
            token,
        }
    }
}