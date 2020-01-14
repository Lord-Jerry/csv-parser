use lexer::Lexer;
use parser::Parser;

fn main() {
    let tokens = Lexer::new(
        "Name,Email,Phone Number,Address
        Bob Smith,bob@example.com,123-456-7890,123 Fake Street
        Mike Jones,mike@example.com,098-765-4321,321 Fake Avenue"
            .to_string(),
    )
    .lex();
    // println!("{:?}", tokens);
    let output = Parser::new(tokens).parse_all();
    println!("{:#?}", output);
}
