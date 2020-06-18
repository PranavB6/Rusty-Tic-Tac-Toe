use super::*;

#[derive(Debug, Clone)]
pub enum Winner {
    Token(Token),
    Tie,
}

impl From<Token> for Winner {
    fn from(original: Token) -> Self {
        match original {
            Token::X => Winner::Token(Token::X),
            Token::O => Winner::Token(Token::O),
        }
    }
}
