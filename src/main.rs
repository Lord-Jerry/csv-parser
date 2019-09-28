use lexer::{ Lexer };

fn main() {
    Lexer::new("name, age, work, cars".to_string()).lex();
    // println!("Hello, world!");

}
 