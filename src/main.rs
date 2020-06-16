mod board;
mod player;
mod utils;

use board::Board;
use player::Player;
use utils::*;

fn main() {
    let mut board = Board::new();
    let player1 = Player::new(Token::X);
    let player2 = Player::new(Token::O);
    let mut turn = Turn::First;

    clear();
    board.draw();
    while !board.game_over() {
        let (pos, token) = match turn {
            Turn::First => player1.choose_move(),
            Turn::Second => player2.choose_move(),
        };

        clear();

        match board.make_move(pos, token) {
            Ok(()) => turn.next(),
            Err(e) => println!("Err: {}\n", e),
        }

        board.draw();
    }

    match board.winner() {
        Winner::Token(token) => println!("{} Wins!", token),
        Winner::Tie => println!("It's a Tie!"),
    }
}
