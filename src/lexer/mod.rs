use crate::token::{Token, TokenType};

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

fn skip_whitespace(ch: u8, lexer: &mut Lexer) {
    if ch.is_ascii_whitespace() {
        lexer.read_char();
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char(); // Init by reading the first char of the input
        lexer
    }
    // When encountering an Identifier, this will read to the end of the word.
    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        skip_whitespace(self.ch, self);
        let token = match self.ch {
            b'=' => Token::new(TokenType::Assign, self.ch.to_string()),
            b';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            b'(' => Token::new(TokenType::LParen, self.ch.to_string()),
            b')' => Token::new(TokenType::RParen, self.ch.to_string()),
            b',' => Token::new(TokenType::Comma, self.ch.to_string()),
            b'+' => Token::new(TokenType::Plus, self.ch.to_string()),
            b'{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            b'}' => Token::new(TokenType::RBrace, self.ch.to_string()),
            0 => Token::new(TokenType::Eof, "".to_string()),
            _ => {
                if is_letter(self.ch) {
                    // read the ID, and return it as token
                    // we must expicitly return here to prevent skipping
                    // read_char is called right after this which advances our pointer
                    // read_identifier advances to the proper next token after execution
                    return Token::new(TokenType::Ident, self.read_identifier());
                } else {
                    return Token::new(TokenType::Illegal, self.ch.to_string());
                }
            }
        };
        self.read_char();
        token
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        println!("{:#?}", token);
        if token.token_type == TokenType::Eof {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_creation() {
        let lexer = Lexer::new("let x = 5;".to_string());
        assert_eq!(lexer.ch, b'l');
    }

    #[test]
    fn test_read_char() {
        let mut lexer = Lexer::new("abc".to_string());
        assert_eq!(lexer.ch, b'a');

        lexer.read_char();
        assert_eq!(lexer.ch, b'b');

        lexer.read_char();
        assert_eq!(lexer.ch, b'c');

        lexer.read_char();
        assert_eq!(lexer.ch, 0); //Eof
    }

    #[test]
    fn test_next_token() {
        //         let input = r#"let five = 5;
        // let ten = 10;
        // let add = fn(x, y) {
        //     x + y;
        // };
        // let result = add(five, ten);"#;
        let input = r#"x , y"#;
        println!("{}", input);
        let lexer = Lexer::new(input.to_string());

        let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
        println!("{:#?}", tokens);

        //     assert_eq!(
        //         tokens,
        //         vec![
        //             TokenType::Let,
        //             TokenType::Ident,
        //             TokenType::Assign,
        //             TokenType::Int,
        //             TokenType::Semicolon,
        //             TokenType::Let,
        //             TokenType::Ident,
        //             TokenType::Assign,
        //             TokenType::Int,
        //             TokenType::Semicolon,
        //             TokenType::Let,
        //             TokenType::Ident,
        //             TokenType::Assign,
        //             TokenType::Function,
        //             TokenType::LParen,
        //             TokenType::Ident,
        //             TokenType::Comma,
        //             TokenType::Ident,
        //             TokenType::RParen,
        //             TokenType::LBrace,
        //             TokenType::Ident,
        //             TokenType::Plus,
        //             TokenType::Ident,
        //             TokenType::Semicolon,
        //             TokenType::RBrace,
        //             TokenType::Semicolon,
        //             TokenType::Let,
        //             TokenType::Ident,
        //             TokenType::Assign,
        //             TokenType::Ident,
        //             TokenType::LParen,
        //             TokenType::Ident,
        //             TokenType::Comma,
        //             TokenType::Ident,
        //             TokenType::RParen,
        //             TokenType::Semicolon,
        //             TokenType::Eof
        //         ]
        //     );
    }
}
