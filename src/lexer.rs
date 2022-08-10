#[derive(Debug)]
pub struct Lexer {
    pub name: String,
}
impl Lexer {
    #[allow(dead_code)]
    pub fn new(name: String) -> Lexer {
        Lexer { name }
    }
}
