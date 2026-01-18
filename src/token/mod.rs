mod token_type;

// re-export for use
pub use token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<String>) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_literal = match self.token_type {
            TokenType::Ident | TokenType::Int | TokenType::Illegal => {
                self.literal.as_deref().unwrap_or("literal was none")
            }
            _ => self.token_type.as_str(),
        };
        write!(f, "Token{:?} '{}'", self.token_type, display_literal)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Int, Some("5".to_string()));
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.literal.as_deref(), Some("5"));
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Int, Some("42".to_string()));
        let display = format!("{}", token);
        assert!(display.contains("Int"));
        assert!(display.contains("42"));
    }
}
