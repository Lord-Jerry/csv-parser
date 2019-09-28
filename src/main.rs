use lexer::{ Lexer };
use parser::{ Parser };

fn main() {
   let tokens = Lexer::new("name, age, 1, \"you\", cars
    ;".to_string()).lex();
    // println!("{:?}", tokens);
    Parser::new(tokens).parse_tokens();
}
 