pub enum Turn {
    First,
    Second,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::First => *self = Turn::Second,
            Turn::Second => *self = Turn::First,
        }
    }

    pub fn random() -> Turn {
        if rand::random() {
            Turn::First
        } else {
            Turn::Second
        }
    }
}
