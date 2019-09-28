use lexer::kinds::{TokenKind, Token};

#[allow(dead_code)]
pub struct Parser {
    // source code
    tokens: Vec<Result<Token, Token>>,
    // Parser Position
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Result<Token, Token>>) -> Self {
        Parser{
            tokens,
            position: 0,
        }
    }

     // check if parser position does not exceed token length
    fn is_bound(&self) -> bool {

        if self.position < self.tokens.len() {
            return true;
        }

        false
    }

    // return token in current parser position
    fn peek_token(&self) -> Option<&Result<Token, Token>> {

        // check if lexer position hasn't exceeded code length
        if self.is_bound() {
            return Some(&self.tokens[self.position]);
        }

        None
    }

    // if error occurs while parsing we would want to revert parser position to position before error
    fn revert(&mut self, position: usize) {
        self.position = position;
    }

    // return current character in lexer position and increment position
    fn eat_token(&mut self) -> &Result<Token, Token> {
        self.position += 1;

        &self.tokens[self.position - 1]
    }

    pub fn parse_tokens(&mut self) {
        while self.is_bound() {
            println!("{:?}", self.eat_token())
        }
     }

}