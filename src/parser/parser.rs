use lexer::kinds::{TokenKind, Token};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Parser {
    // source code
    tokens: Vec<Result<Token, Token>>,
    // Parser Position
    position: usize,
    header: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Result<Token, Token>>) -> Self {
        Parser{
            tokens,
            position: 0,
            header: vec![],
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
    fn peek_token(&self) -> Option<&Token> {

        // check if lexer position hasn't exceeded code length
        if self.is_bound() {
            return Some(self.tokens[self.position].as_ref().unwrap());
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

    fn parse_header(&mut self) {
        let mut header = vec![];
        let mut temp;

        // get CSV headers
        println!("{:?}  test", self.position);
        while self.is_bound() && (self.peek_token().unwrap().kind != TokenKind::Newline) {
            temp = &self.tokens[self.position].as_ref().unwrap().token;
            let kind = &self.tokens[self.position].as_ref().unwrap().kind;
            self.position += 1;
            println!("{:?}", self.position);

            if kind == &TokenKind::Identifier {
                header.push(temp.to_string());
            }
        }

        if self.peek_token().unwrap().kind == TokenKind::Newline {
            self.eat_token();
        }

        println!("{:?}", header);
        self.header = header;

        self.parse_body();

    }

    fn parse_body(&mut self) {
        let header_count = self.header.len();
        let mut count = 0;
        let mut group = HashMap::new();
        let mut body = vec![];
        print!("11");

        while self.is_bound() && (self.peek_token().unwrap().kind != TokenKind::Newline) {
            let temp = &self.tokens[self.position].as_ref().unwrap().token;
            let kind = &self.tokens[self.position].as_ref().unwrap().kind;
            self.position += 1;

            if body.len() >= self.header.len() {
                panic!("error expected newline but got {:?}", temp);
            } else if kind == &TokenKind::Identifier {
                if count < header_count {

                   body.push(group.insert(
                        &self.header[count],
                        temp.to_string(),
                    ));
                    count += 1;
                }
            }
        }

        if self.peek_token().unwrap().kind == TokenKind::Newline {
            self.eat_token();
        }

        println!("{:?}", body);


    }

    pub fn parse_all(&mut self) {
        println!("{:?}",self.tokens);
        while self.is_bound() {
            // println!("{:?}", self.eat_token());
            self.parse_header();
            // self.parse_body();
        }
     }

}