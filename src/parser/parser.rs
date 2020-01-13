use lexer::kinds::{Token, TokenKind};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Parser {
    // source code
    tokens: Vec<Result<Token, Token>>,
    // Parser Position
    position: usize,
    header: Vec<String>,
    output: Vec<HashMap<String, String>>,
}

impl Parser {
    pub fn new(tokens: Vec<Result<Token, Token>>) -> Self {
        Parser {
            tokens,
            position: 0,
            header: vec![],
            output: vec![],
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

    // return current character in lexer position and increment position
    fn eat_token(&mut self) -> &Token {
        self.position += 1;

        self.tokens[self.position - 1].as_ref().unwrap()
    }

    fn parse_header(&mut self) {
        let mut header = vec![];
        let mut temp;

        while self.is_bound() && (self.peek_token().unwrap().kind != TokenKind::Newline) {
            let token = &self.eat_token();
            temp = &token.token;
            let kind = &token.kind;

            if kind == &TokenKind::Identifier {
                header.push(temp.to_string());
            }
        }

        if self.peek_token().unwrap().kind == TokenKind::Newline {
            self.eat_token();
        }

        self.header = header;

        // self.parse_body();
    }

    fn parse_body(&mut self) {
        let header = &self.header.clone();
        let mut count = 0;
        let mut group = HashMap::new();

        while self.is_bound() {
            if self.peek_token().unwrap().kind == TokenKind::Newline {
                self.eat_token();
                break;
            }

            // println!("{:?}", self.peek_token());
            let token = &self.eat_token();
            let temp = &token.token;
            let kind = &token.kind;

            if kind == &TokenKind::Identifier {
                if count < header.len() {
                    group.insert(header[count].to_string(), temp.to_string());
                    count += 1;
                }
            }
        }

        self.output.push(group);
    }

    pub fn parse_all(&mut self) -> Vec<HashMap<String, String>> {
        // println!("{:?}", self.tokens);
        self.parse_header();
        while self.is_bound() {
            self.parse_body();
        }

        self.output.clone()
    }
}
