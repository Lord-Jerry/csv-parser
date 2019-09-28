use lexer::{ Lexer };
use parser::{ Parser };

fn main() {
   let tokens = Lexer::new(
    "name, age, 1, \"you\", cars,
     jerry, 18, hey, yes , 20".to_string()).lex();
    // println!("{:?}", tokens);
    Parser::new(tokens).parse_all();
}
 