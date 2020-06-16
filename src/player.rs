use std::io::{stdout, Write};

use crate::board::Position;
use crate::utils::*;

pub struct Player {
    token: Token,
}

impl Player {
    pub fn new(token: Token) -> Self {
        Self { token }
    }

    pub fn choose_move(&self) -> (Position, Token) {
        let mut input;

        loop {
            input = get_input(&format!("{} >> ", self.token));

            if let Ok(pos) = input.parse::<Position>() {
                return (pos, self.token.clone());
            } else {
                println!("Err: Invalid position");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error in getting user input");

    input.trim().to_string()
}
