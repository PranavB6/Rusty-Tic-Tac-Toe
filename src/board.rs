use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Board {
    board: RefCell<HashMap<Position, Token>>,
    size: Position,
    winner: RefCell<Option<Winner>>,
    winning_configs: [[Position; 3]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: RefCell::new(HashMap::new()),
            size: 9,
            winner: RefCell::new(None),
            winning_configs: [
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9],
                [1, 4, 7],
                [2, 5, 8],
                [3, 6, 9],
                [1, 5, 9],
                [3, 5, 7],
            ],
        }
    }

    pub fn make_move(&self, pos: Position, token: Token) -> Result<(), &'static str> {
        if pos <= 0 || pos > self.size {
            Err("Position out of bounds")
        } else if self.has_token(pos) {
            Err("There is already a token at that position")
        } else {
            self.insert_token_at(pos, token);
            self.check();
            Ok(())
        }
    }

    pub fn game_over(&self) -> bool {
        self.winner.borrow().is_some()
    }

    pub fn winner(&self) -> Winner {
        let winner = self.winner.borrow().clone();

        winner.expect("Game is not over yet, there is no winner")
    }

    pub fn check(&self) {
        if self.is_full() {
            *self.winner.borrow_mut() = Some(Winner::Tie);
        }

        for config in &self.winning_configs {
            let [a, b, c] = *config;
            if self.has_token(a)
                && (self.get_token_at(a) == self.get_token_at(b))
                && self.get_token_at(b) == self.get_token_at(c)
            {
                let token = self.get_token_at(a).unwrap().clone();
                *self.winner.borrow_mut() = Some(Winner::from(token));
            }
        }
    }

    pub fn get_empty_positions(&self) -> Vec<Position> {
        let mut empty_positions = vec![];

        for pos in 1..=self.size {
            if !self.has_token(pos) {
                empty_positions.push(pos);
            }
        }

        empty_positions
    }

    pub fn is_empty(&self) -> bool {
        self.board.borrow().is_empty()
    }

    pub fn draw(&self) {
        let token_at = |pos| {
            self.get_token_at(pos)
                .map(|token| token.to_string())
                .unwrap_or(" ".to_string())
        };

        println!("1   |2   |3   ");
        println!(" {}  | {}  | {}  ", token_at(1), token_at(2), token_at(3));
        println!("----|----|----");
        println!("4   |5   |6   ");
        println!(" {}  | {}  | {}  ", token_at(4), token_at(5), token_at(6));
        println!("----|----|----");
        println!("7   |8   |9   ");
        println!(" {}  | {}  | {}  ", token_at(7), token_at(8), token_at(9));
        println!("    |    |    ");
    }

    fn is_full(&self) -> bool {
        for pos in 1..=self.size {
            if !self.has_token(pos) {
                return false;
            }
        }

        return true;
    }

    fn get_token_at(&self, pos: Position) -> Option<Token> {
        self.board
            .borrow()
            .get(&pos)
            .map(|token_ref| token_ref.clone())
    }

    fn has_token(&self, pos: Position) -> bool {
        self.get_token_at(pos).is_some()
    }

    fn insert_token_at(&self, pos: Position, token: Token) {
        let prev_val = self.board.borrow_mut().insert(pos, token);
        assert!(prev_val.is_none(), "There is already a token there!");
    }
}
