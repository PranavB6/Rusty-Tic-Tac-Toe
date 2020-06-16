use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    X,
    O,
}

impl Token {
    fn flip(&self) -> Token {
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

#[derive(Debug)]
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

pub enum Turn {
    First,
    Second,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::First => *self = Turn::Second,
            Turn::Second => *self = Turn::First,
        }
    }
}

pub fn clear() {
    // Clears the terminal
    print!("\x1B[2J\x1B[1;1H");
}