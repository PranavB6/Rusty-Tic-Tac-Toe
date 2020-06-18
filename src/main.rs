use dialoguer::{theme::ColorfulTheme, Select};
use tic_tac_toe::*;

fn main() {
    let board = Board::new();
    let player1: Box<dyn Player>;
    let player2: Box<dyn Player>;

    let mode = choose_game_mode();

    match &mode {
        GameMode::VsHuman => {
            player1 = Box::new(Human::new(Token::X, "Player 1"));
            player2 = Box::new(Human::new(Token::O, "Player 2"));
        }
        GameMode::VsBot => {
            let difficulty = choose_difficulty();
            let token = choose_token();
            player1 = Box::new(Human::new(token.clone(), "Player"));
            player2 = Box::new(Bot::new(token.flip(), difficulty, &board));
        }
    }

    let mut turn = Turn::random();

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

fn choose_game_mode() -> GameMode {
    let modes = ["Human vs Human", "Human vs Bot"];
    let selected_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose Mode")
        .default(0)
        .items(&modes[..])
        .interact()
        .unwrap();

    match selected_mode {
        0 => GameMode::VsHuman,
        1 => GameMode::VsBot,
        _ => panic!("Invalid game mode chosen, idk how you even did this"),
    }
}

fn choose_difficulty() -> Difficulty {
    let difficulties = &["Easy", "Normal", "Impossible"];
    let selected_difficulty = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose Difficulty")
        .default(0)
        .items(&difficulties[..])
        .interact()
        .unwrap();

    match selected_difficulty {
        0 => Difficulty::Easy,
        1 => Difficulty::Normal,
        2 => Difficulty::Impossible,
        _ => panic!("Invalid difficulty chosen, idk how you even did this"),
    }
}

fn choose_token() -> Token {
    let tokens = &["X", "O"];
    let selected_token = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose Difficulty")
        .default(0)
        .items(&tokens[..])
        .interact()
        .unwrap();

    match selected_token {
        0 => Token::X,
        1 => Token::O,
        _ => panic!("Invalid difficulty chosen, idk how you even did this"),
    }
}
