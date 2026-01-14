mod lexer;
mod token;
use crate::token::{Token, TokenType};

fn main() {
    let token = Token::new(TokenType::Plus, "+".to_string());
    println!("{}", token);
}
