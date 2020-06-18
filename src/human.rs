use crate::utils::*;

pub struct Human {
    token: Token,
    name: String,
}

impl Human {
    pub fn new(token: Token, name: &str) -> Self {
        Self {
            token,
            name: name.to_owned(),
        }
    }
}

impl Player for Human {
    fn choose_move(&self) -> (Position, Token) {
        let mut input;

        loop {
            input = get_input(&format!("{} ({}) >> ", self.name, self.token));

            if let Ok(pos) = input.parse::<Position>() {
                return (pos, self.token.clone());
            } else {
                println!("Err: Invalid position");
            }
        }
    }
}
