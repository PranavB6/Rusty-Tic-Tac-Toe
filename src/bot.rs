use crate::board::*;
use crate::utils::*;
use rand::seq::SliceRandom;

pub struct Bot<'a> {
    token: Token,
    difficulty: Difficulty,
    board: &'a Board,
}

impl<'a> Bot<'a> {
    pub fn new(token: Token, difficulty: Difficulty, board: &'a Board) -> Self {
        Self {
            token,
            difficulty,
            board,
        }
    }
    pub fn calc_best_pos(&self) -> Position {
        let (_, opt_pos) = self.minimax(&self.board, true);
        opt_pos.unwrap()
    }

    pub fn minimax(&self, board: &Board, maximizing: bool) -> (f64, Option<Position>) {
        if board.game_over() {
            // If the bot wins it gets 10 points, if the other player wins, it gets -10 points, tie is 0 points
            let points = match board.winner() {
                Winner::Token(winner) if self.token == winner => 10.0,
                Winner::Token(_) => -10.0,
                Winner::Tie => 0.0,
            };

            return (points, None);
        }

        let empty_positions = board.get_empty_positions();

        if maximizing {
            let mut max_points = std::f64::NEG_INFINITY;
            let mut best_pos = empty_positions[0];

            for pos in empty_positions {
                let mut board_clone = board.clone();
                board_clone.make_move(pos, self.token.clone()).unwrap();

                let (points, _) = self.minimax(&board_clone, false);

                if points > max_points {
                    max_points = points;
                    best_pos = pos;
                }
            }

            return (max_points, Some(best_pos));
        } else {
            let mut min_points = std::f64::INFINITY;
            let mut best_pos = empty_positions[0];

            for pos in empty_positions {
                let mut board_clone = board.clone();
                board_clone.make_move(pos, self.token.flip()).unwrap();

                let (points, _) = self.minimax(&board_clone, true);

                if points < min_points {
                    min_points = points;
                    best_pos = pos;
                }
            }

            return (min_points, Some(best_pos));
        }
    }
}

impl Player for Bot<'_> {
    fn choose_move(&self) -> (Position, Token) {
        let random_pos = || {
            *self
                .board
                .get_empty_positions()
                .choose(&mut rand::thread_rng())
                .unwrap()
        };

        let pos = match &self.difficulty {
            Difficulty::Easy => random_pos(),
            Difficulty::Normal => {
                if rand::random() {
                    random_pos()
                } else {
                    self.calc_best_pos()
                }
            }
            Difficulty::Impossible => self.calc_best_pos(),
        };

        (pos, self.token.clone())
    }
}
