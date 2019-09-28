use lexer::{ Lexer };

fn main() {
   let tokens = Lexer::new("name, age, 1, \"you\", cars
    ".to_string()).lex();
    println!("{:?}", tokens);

}
 