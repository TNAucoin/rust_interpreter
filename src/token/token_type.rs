#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EndOfFile",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_as_str() {
        assert_eq!(TokenType::Plus.as_str(), "+");
        assert_eq!(TokenType::Comma.as_str(), ",");
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Int, TokenType::Int);
        assert_eq!(TokenType::Illegal, TokenType::Illegal);
    }
}
