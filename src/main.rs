#![allow(dead_code)]

mod board;
mod bot;
mod player;
mod utils;

use board::*;
use bot::*;
use player::*;
use utils::Token::*;
use utils::*;

fn main() {
    println!("Hello, world!");

    let mut board = Board::new();
    let mut bot = Bot::new(Token::X, &board);

    // let mut board = Board::new();
    // let mut player1 = Player::new(X);
    // let mut player2 = Player::new(O);
    // let mut turn = Turn::First;

    // clear();
    // board.draw();
    // while !board.game_over() {
    //     let (pos, token) = match turn {
    //         Turn::First => player1.choose_move(),
    //         Turn::Second => player2.choose_move(),
    //     };

    //     clear();

    //     match board.make_move(pos, token) {
    //         Ok(()) => turn.next(),
    //         Err(e) => println!("Err: {}\n", e),
    //     }

    //     board.draw();
    // }

    // match board.winner() {
    //     Winner::Token(token) => println!("{} Wins!", token),
    //     Winner::Tie => println!("It's a Tie!"),
    // }
}
