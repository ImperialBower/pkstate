//! Game type and forced bet configuration.
//!
//! This module provides:
//! - [`GameType`] — the poker variant being played.
//! - [`ForcedBets`] — the mandatory bets (blinds, straddles, ante) required to start a hand.

use serde::{Deserialize, Serialize};

/// The mandatory bets required before cards are dealt.
///
/// At minimum a hand requires a small blind and a big blind. Straddles and an ante are optional.
///
/// # Examples
///
/// ```rust
/// use pkstate::game::ForcedBets;
///
/// let blinds_only = ForcedBets::new(50, 100);
/// let with_ante   = ForcedBets::new_with_ante(50, 100, 200);
/// let with_straddles = ForcedBets::new_with_straddles(50, 100, vec![200, 400]);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct ForcedBets {
    /// The small blind amount.
    pub small: usize,
    /// The big blind amount.
    pub big: usize,
    /// Optional list of straddle amounts in posting order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straddles: Option<Vec<usize>>,
    /// Optional ante amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ante: Option<usize>,
}

impl ForcedBets {
    /// Creates a [`ForcedBets`] with only a small and big blind.
    #[must_use]
    pub fn new(small: usize, big: usize) -> Self {
        ForcedBets {
            small,
            big,
            straddles: None,
            ante: None,
        }
    }

    /// Creates a [`ForcedBets`] with a small blind, big blind, and one or more straddles.
    #[must_use]
    pub fn new_with_straddles(small: usize, big: usize, straddles: Vec<usize>) -> Self {
        ForcedBets {
            small,
            big,
            straddles: Some(straddles),
            ante: None,
        }
    }

    /// Creates a [`ForcedBets`] with a small blind, big blind, and an ante.
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

/// The poker variant being played.
///
/// The variant determines how many hole cards each player receives and how many community
/// cards appear on the board.
///
/// | Variant         | Hole cards | Board cards |
/// |-----------------|-----------|-------------|
/// | `NoLimitHoldem` | 2         | 5           |
/// | `LimitHoldem`   | 2         | 5           |
/// | `PLO`           | 4         | 0           |
/// | `Razz`          | 7         | 0           |
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq,
)]
pub enum GameType {
    /// No-Limit Texas Hold'em (default).
    #[default]
    NoLimitHoldem,
    /// Limit Texas Hold'em.
    LimitHoldem,
    /// Pot-Limit Omaha.
    PLO,
    /// Razz (7-card stud low).
    Razz,
}

impl GameType {
    /// Returns the number of hole cards dealt to each player for this game type.
    #[must_use]
    pub fn cards_per_player(&self) -> u8 {
        match self {
            GameType::LimitHoldem | GameType::NoLimitHoldem => 2,
            GameType::PLO => 4,
            GameType::Razz => 7,
        }
    }

    /// Returns the number of community board cards for this game type.
    #[must_use]
    pub fn cards_on_board(&self) -> u8 {
        match self {
            GameType::NoLimitHoldem => 5,
            _ => 0,
        }
    }

    /// Returns the standard deck size used for this game type (always 52).
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
