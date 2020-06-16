#![allow(dead_code)]

use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fmt;

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
    let term = Term::buffered_stderr();

    let player_name: String = Input::new().with_prompt("Name").interact_on(&term).unwrap();

    let difficulties = &["Easy", "Normal", "Hard"];
    let difficulty = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose Difficulty")
        .default(0)
        .items(&difficulties[..])
        .interact()
        .unwrap();

    let player1 = Player::new(Token::X, &player_name);
    // let player2 = Player::new(O, "player 2");
    let bot = Bot::new(Token::O, difficulty);
    let mut board = Board::new();

    let mut turn = Turn::First;

    clear();
    board.draw();
    while !board.game_over() {
        let (pos, token) = match turn {
            Turn::First => player1.choose_move(),
            Turn::Second => bot.choose_move(&board),
            // Turn::Second => player2.choose_move(),
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
