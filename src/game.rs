use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum GameType {
    #[default]
    NoLimitHoldem,
    LimitHoldem,
    PLO,
    Razz,
}

impl GameType {
    #[must_use]
    pub fn cards_per_player(&self) -> u8 {
        match self {
            GameType::LimitHoldem | GameType::NoLimitHoldem => 2,
            GameType::PLO => 4,
            GameType::Razz => 7,
        }
    }

    #[must_use]
    pub fn cards_on_board(&self) -> u8 {
        match self {
            GameType::NoLimitHoldem => 5,
            _ => 0,
        }
    }

    #[must_use]
    pub fn get_deck_size(&self) -> usize {
        52
    }
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::LimitHoldem => write!(f, "Limit Hold'em"),
            GameType::NoLimitHoldem => write!(f, "No Limit Hold'em"),
            GameType::PLO => write!(f, "Pot Limit Omaha"),
            GameType::Razz => write!(f, "Razz"),
        }
    }
}