use crate::board::*;
use crate::utils::*;
use std::collections::HashSet;

pub struct Bot {
    token: Token,
}

impl Bot {
    pub fn new(token: Token) -> Self {
        Self { token }
    }

    pub fn choose_move(&self, board: &Board) -> (Position, Token) {
        let (points, opt_pos) = self.minimax(&board, true);

        (opt_pos.unwrap(), self.token.clone())
    }

    pub fn minimax(&self, board: &Board, maximizing: bool) -> (f64, Option<Position>) {
        if board.game_over() {
            let points = match board.winner() {
                Winner::Token(winner) if self.token == winner => 10.0,
                Winner::Token(winner) => -10.0,
                Winner::Tie => 0.0,
            };

            // println!(
            //     "Game Over. Points: {}, Winner: {:?}",
            //     points,
            //     board.winner()
            // );

            return (points, None);
        }

        let mut empty_positions = board.get_empty_positions();

        if maximizing {
            let mut max_points = std::f64::NEG_INFINITY;
            let mut best_pos = &empty_positions[0];

            for pos in &empty_positions {
                // println!("Testing position: {}", pos);
                let mut board_clone = board.clone();
                board_clone.make_move(*pos, self.token.clone()).unwrap();

                let (points, _) = self.minimax(&board_clone, false);

                if points > max_points {
                    max_points = points;
                    best_pos = &pos;
                }
            }

            // println!("Max Points: {}, Best Pos: {}", max_points, best_pos);
            return (max_points, Some(*best_pos));
        } else {
            let mut min_points = std::f64::INFINITY;
            let mut best_pos = &empty_positions[0];

            for pos in &empty_positions {
                // println!("Testing position: {}", pos);
                let mut board_clone = board.clone();
                board_clone.make_move(*pos, self.token.flip()).unwrap();

                let (points, _) = self.minimax(&board_clone, true);

                if points < min_points {
                    min_points = points;
                    best_pos = &pos;
                }
            }

            return (min_points, Some(*best_pos));
        }

        // let mut empty_spots = board.get_empty_positions();

        // if maximizing {
        //     let mut max_points = std::f64::NEG_INFINITY;
        //     let mut best_pos = empty_spots[0];

        //     for pos in empty_spots {
        //         let mut new_board = board.clone();
        //         new_board.make_move(pos, self.token.clone()).unwrap();
        //         let (points, memo) = self.minimax(&new_board, memo.clone(), false);

        //         if points > max_points {
        //             max_points = points;
        //             best_pos = pos;
        //         }
        //     }

        //     if first_lvl {
        //         memo.insert(best_pos);
        //     }
        // }

        // return (-10.0, Some(8));
    }
}
