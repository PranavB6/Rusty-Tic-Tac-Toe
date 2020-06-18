use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    X,
    O,
}

#[allow(dead_code)]
impl Token {
    pub fn flip(&self) -> Token {
        match self {
            Token::X => Token::O,
            Token::O => Token::X,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::X => write!(f, "X"),
            Token::O => write!(f, "O"),
        }
    }
}
