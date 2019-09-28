#[allow(dead_code)]
pub struct Parser {
    // source code
    code: Vec<Result>,
    // Parser Position
    position: usize,
}

impl Parser {
    pub fn new(token: Vec) -> Self {
        Parser{
            code: token,
            position: 0,
        }
    }
}