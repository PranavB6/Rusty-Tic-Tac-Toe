use super::*;

pub trait Player {
    fn choose_move(&self) -> (Position, Token);
}
