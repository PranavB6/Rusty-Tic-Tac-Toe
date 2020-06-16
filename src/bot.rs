use crate::board::*;
use crate::utils::*;
use std::collections::HashSet;

pub struct Bot<'a> {
    token: Token,
    board: &'a Board,
}

impl<'a> Bot<'a> {
    pub fn new(token: Token, board: &'a Board) -> Self {
        Self { token, board }
    }

    pub fn choose_move(&self) {
        self.minimax(self.board, HashSet::new(), true);
    }

    pub fn minimax(
        &self,
        board: &Board,
        memo: HashSet<Position>,
        maximizing: bool,
    ) -> (f64, HashSet<Position>) {
        let mut first_lvl = false;

        if memo.is_empty() {
            first_lvl = true;
        }

        if board.game_over() {
            let points = match board.winner() {
                Winner::Token(winner) if self.token == winner => 10.0,
                Winner::Token(winner) => -10.0,
                Winner::Tie => 0.0,
            };

            return (points, memo);
        }

        let mut empty_spots = board.get_empty_spots();

        if maximizing {
            let mut max_points = std::f64::NEG_INFINITY;
            let mut best_pos = empty_spots[0];

            for pos in empty_spots {
                let mut new_board = board.clone();
                new_board.make_move(pos, self.token.clone()).unwrap();
                let (points, memo) = self.minimax(&new_board, memo.clone(), false);

                if points > max_points {
                    max_points = points;
                    best_pos = pos;
                }
            }

            if first_lvl {
                memo.insert(best_pos);
            }
        }

        return (10.0, memo);
    }
}
