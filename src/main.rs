use lexer::Lexer;
use parser::Parser;

fn main() {
    let tokens = Lexer::new(
    "name, age, location, cars,
     jerry, 18, lagos, yes , 20,
     lord, 20, ekiti, no, 78
     jade, 20, akure, no, 78"
            .to_string(),
    )
    .lex();
    // println!("{:?}", tokens);
    Parser::new(tokens).parse_all();
}
