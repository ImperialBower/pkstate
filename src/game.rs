use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct ForcedBets {
    pub small: usize,
    pub big: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straddles: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ante: Option<usize>,
}

impl ForcedBets {
    #[must_use]
    pub fn new(small: usize, big: usize) -> Self {
        ForcedBets {
            small,
            big,
            straddles: None,
            ante: None,
        }
    }

    #[must_use]
    pub fn new_with_straddles(small: usize, big: usize, straddles: Vec<usize>) -> Self {
        ForcedBets {
            small,
            big,
            straddles: Some(straddles),
            ante: None,
        }
    }

    #[must_use]
    pub fn new_with_ante(small: usize, big: usize, ante: usize) -> Self {
        ForcedBets {
            small,
            big,
            straddles: None,
            ante: Some(ante),
        }
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq,
)]
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
