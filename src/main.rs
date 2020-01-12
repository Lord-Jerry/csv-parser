use lexer::Lexer;
use parser::Parser;

fn main() {
    let tokens = Lexer::new(
        "name, age, location,
     jerry, 18, lagos, yes,
     lord, 20, ekiti, no,
     jade, 20, akure, no,"
            .to_string(),
    )
    .lex();
    // println!("{:?}", tokens);
    Parser::new(tokens).parse_all();
}
