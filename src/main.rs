use lexer::{ Lexer };

fn main() {
    Lexer::new("name, age, 1, \"you\", cars
    ".to_string()).lex();
    // println!("Hello, world!");

}
 