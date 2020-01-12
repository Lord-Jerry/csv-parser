use lexer::kinds::{Token, TokenKind};
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
        Parser {
            tokens,
            position: 0,
            header: vec![],
        }
    }

    // check if parser position does not exceed token length
    fn is_bound(&self) -> bool {
        if self.position <= self.tokens.len() - 1 {
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

        while self.is_bound() && (self.peek_token().unwrap().kind != TokenKind::Newline) {
            temp = &self.tokens[self.position].as_ref().unwrap().token;
            let kind = &self.tokens[self.position].as_ref().unwrap().kind;
            self.position += 1;

            if kind == &TokenKind::Identifier {
                header.push(temp.to_string());
            }
        }

        if self.peek_token().unwrap().kind == TokenKind::Newline {
            self.eat_token();
        }

        println!("{:?}", header);
        self.header = header;

        // self.parse_body();
    }

    fn parse_body(&mut self) {
        let header = &self.header;
        let mut count = 0;
        let mut group = HashMap::new();
        let mut body = vec![];

        while self.is_bound() {
            if self.peek_token().unwrap().kind == TokenKind::Newline {
                self.position += 1;
                break;
            }

            // println!("{:?}", self.peek_token());
            let token = &self.tokens[self.position].as_ref();
            let temp = &token.unwrap().token;
            let kind = &token.unwrap().kind;
            self.position += 1;

            if body.len() >= header.len() {
                panic!("error expected newline but got {:?}", temp);
            } else if kind == &TokenKind::Identifier {
                if count < header.len() {
                    &group.insert(&header[count], temp.to_string());
                    count += 1;
                }
            }
        }

        body.push(&group);

        println!("{:?}", body);
    }

    pub fn parse_all(&mut self) {
        // println!("{:?}", self.tokens);
        self.parse_header();
        while self.is_bound() {
            self.parse_body();
        }
    }
}
