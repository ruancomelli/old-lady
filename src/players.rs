use ggez::graphics::Color;
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Player::X => Color::RED,
            Player::O => Color::BLUE,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}
