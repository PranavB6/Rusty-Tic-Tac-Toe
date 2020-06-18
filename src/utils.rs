use std::io::Write;
mod player;
mod token;
mod turn;
mod winner;

pub use player::Player;
pub use token::Token;
pub use turn::Turn;
pub use winner::Winner;

pub type Position = i8;

pub enum GameMode {
    VsHuman,
    VsBot,
}

pub enum Difficulty {
    Easy,
    Normal,
    Impossible,
}

pub fn clear() {
    // Clears the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error in getting user input");

    input.trim().to_string()
}
