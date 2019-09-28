pub mod kinds;
use kinds::{ TokenKind, Token };

#[allow(dead_code)]
pub struct Lexer {
    // source code
    code: Vec<char>,
    // Lexer position
    position: usize,

    column: usize,
    line: usize,
    // supported space characters
    space_char: String,
    // supported integer literals
    digit_char: String,
    // supported characters that can an identifier
    identifier_char: String,
    // supported separators
    separator_char: String,
    // supported keywords
}

impl Lexer {

    pub fn new(code: String) -> Self {
        Lexer {
            code: code.chars().collect(),
            // lexer position
            position: 0,

            column: 0,
            line: 1,
            space_char: String::from("\t "),
            digit_char: String::from("0123456789"),
            identifier_char: String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789"),
            separator_char: String::from(","),
        }
    }

     // check if lexer position does not exceed code length
    fn is_bound(&self) -> bool {

        if self.position < self.code.len() {
            return true;
        }

        false
    }

    // return character in current lexer position
    fn peek_char(&self) -> Option<char> {

        // check if lexer position hasn't exceeded code length
        if self.is_bound() {
            return Some(self.code[self.position]);
        }

        None
    }

    // if error occurs while lexing we would want to revert lexer position to position before error
    // this method those that
    fn revert(&mut self, position: usize) {
        self.position = position;
        self.column = position;
    }

    // return current character in lexer position and increment position
    fn eat_char(&mut self) -> char {
        self.position += 1;

        self.code[self.position - 1]
    }

    // check if characters are valid identifier
    fn vaild_identifier(&mut self) -> Token {
        let mut kind = TokenKind::Identifier;
        let mut identifier = String::from("");
        let mut character = self.peek_char();
        let start_position = self.position;
           
        
            // check if lexer position hasn't exceeded code length and if character is a valid aplhabetnumeric characeter
        while self.is_bound() && self.identifier_char.contains(character.unwrap()) {
            identifier.push(self.eat_char());

            // assign new character to character variable 
            character = self.peek_char();
        }
         

        // if no avaliable valid character assign token kind to unknown 
        if identifier.len()  < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;

        Token::new(kind, start_position, end_position, identifier)

    }

    // check if characters are valid identifier
    fn qouted_identifier(&mut self) -> Token {
        let mut kind = TokenKind::Identifier;
        let mut identifier = String::from("");
        let mut character = self.peek_char();
        let start_position = self.position;
           
        // lex single quoted identifier
        if character.unwrap() == '\'' {
            identifier.push(self.eat_char());

            // assign new character to character variable 
            character = self.peek_char();

            // check if lexer position hasn't exceeded code length and if character is in quote
            while self.is_bound() {
                if character.unwrap() == '\'' {
                    identifier.push(self.eat_char());
                    break;
                }
                identifier.push(self.eat_char());

                // assign new character to character variable 
                character = self.peek_char();
            }
        }
        
        // lex double quoted identifier
        if character.unwrap() == '\"' {
            identifier.push(self.eat_char());

            // assign new character to character variable 
            character = self.peek_char();

            // check if lexer position hasn't exceeded code length and if character is in quote
            while self.is_bound() {
                if character.unwrap() == '\"' {
                    identifier.push(self.eat_char());
                    break;
                }
                identifier.push(self.eat_char());

                // assign new character to character variable 
                character = self.peek_char();
            }
        }
        
         

        // if no avaliable valid character assign token kind to unknown 
        if identifier.len()  < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;

        Token::new(kind, start_position, end_position, identifier)

    }


    // check if characeter is an identified separator
    fn valid_separator(&mut self) -> Token {
        let mut separator = String::from("");
        let character = self.peek_char();
        let mut kind = TokenKind::Separator;
        let start_position = self.position;

        if self.is_bound() && self.separator_char.contains(character.unwrap()) {
            separator.push(self.eat_char());
        }

        if separator.len() < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, separator)

    }

    // check if characeter is an identified separator
    fn valid_newline(&mut self) -> Token {
        let mut newline = String::from("");
        let character = self.peek_char();
        let mut kind = TokenKind::Newline;
        let start_position = self.position;

        if self.is_bound() && character.unwrap() == '\n' {
            newline.push(self.eat_char());
        }

        if newline.len() < 1 {
            kind = TokenKind::Unknown;
            self.line += 1;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, newline)

    }

    // check if character is a valid white space
    fn valid_space(&mut self) -> Token {
        let mut space = String::from("");
        let mut character = self.peek_char();
        let mut kind = TokenKind::Space;
        let start_position = self.position;

        // check if lexer position hasn't exceeded code length and if character is valid space character
        while self.is_bound() && self.space_char.contains(character.unwrap()) {
            space.push(self.eat_char());
            character = self.peek_char();
        }

        // if characeter is not a valid space character assign token kind to unknown
        if space.len() < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, space)
    }

    // run all lexer function
    fn lex_next(&mut self) -> Result<Token, Token> {

        let identifier = self.vaild_identifier();
        if identifier.kind != TokenKind::Unknown {
            return Ok(identifier);
        }

        let separator = self.valid_separator();
        if separator.kind != TokenKind::Unknown {
           return Ok(separator);
        }

       let space = self.valid_space();
       if space.kind != TokenKind::Unknown {
            return Ok(space);
        }

        let quoted = self.qouted_identifier();
        if quoted.kind != TokenKind::Unknown {
            return Ok(quoted);
        }

        let newline = self.valid_newline();
        if newline.kind != TokenKind::Unknown {
            return Ok(newline)
        }

       //let comment = self.valid_single_line_comment();
        //if comment.kind != TokenKind::Unknown {
        //    return Ok(comment);
        //}

        Err(Token::new(TokenKind::Unknown, self.position, self.position + 1, self.eat_char().to_string()))
     }


    pub fn lex(&mut self) {

        while self.is_bound() {

            println!("{:#?}",self.lex_next());
        }
       // println!("{:#?}", self.peek_char());
    }

    
}
