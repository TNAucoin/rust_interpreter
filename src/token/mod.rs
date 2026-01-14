mod token_type;

// re-export for use
pub use token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, '{}')", self.token_type, self.literal)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Int, "5".to_string());
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.literal, "5");
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Int, "42".to_string());
        let display = format!("{}", token);
        assert!(display.contains("Int"));
        assert!(display.contains("42"));
    }
}
