//! Actions and betting rounds.
//!
//! This module provides two types:
//!
//! - [`Action`] — a single event that can occur during a hand: dealing cards to a player,
//!   dealing community cards, a player checking, calling/betting/raising, folding, or a
//!   showdown result. Variants cover seats **P0 through P11** (up to 12 players).
//! - [`Round`] — an ordered sequence of [`Action`]s representing one betting street (preflop,
//!   flop, turn, river, …).
//!
//! ## YAML Representation
//!
//! `Action` uses a custom [`serde::Serialize`] / [`serde::Deserialize`] implementation so that
//! [`BasicPile`] card collections are stored as human-readable Unicode card strings rather than
//! nested structures.
//!
//! | Variant             | YAML                      |
//! |---------------------|---------------------------|
//! | `DealCommon(pile)`  | `DealCommon: "9♣ 6♦ 5♥"`  |
//! | `PnDealt(pile)`     | `P0Dealt: "A♠ K♠"`        |
//! | `PnCheck`           | `P0Check`                 |
//! | `PnCBR(amount)`     | `P0CBR: 200`              |
//! | `PnFold`            | `P1Fold`                  |
//! | `PnWins(amount)`    | `P3Wins: 1000`            |
//! | `PnLoses(amount)`   | `P4Loses: 1000`           |
//!
//! ## Example
//!
//! ```rust
//! use pkstate::act::{Action, Round};
//! use cardpack::prelude::*;
//!
//! let round = Round(vec![
//!     Action::P0Dealt(basic!("A♠ K♠")),
//!     Action::P1Dealt(basic!("7♦ 2♣")),
//!     Action::P0CBR(100),
//!     Action::P1Fold,
//!     Action::P0Wins(100),
//! ]);
//!
//! let yaml = serde_yaml_bw::to_string(&round).unwrap();
//! let restored: Round = serde_yaml_bw::from_str(&yaml).unwrap();
//! assert_eq!(round, restored);
//! ```

use cardpack::prelude::{BasicPile, Pile, Standard52};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A single event in a poker hand.
///
/// Each variant is prefixed with the seat number it belongs to (`P0`–`P11`), or `DealCommon`
/// for community cards shared by all players.
///
/// ## Variant groups (per seat)
///
/// | Variant        | Meaning                                         |
/// |----------------|-------------------------------------------------|
/// | `PnDealt(pile)`| The hole cards dealt to player *n*              |
/// | `PnCheck`      | Player *n* checks                               |
/// | `PnCBR(amount)`| Player *n* calls, bets, or raises *amount*      |
/// | `PnFold`       | Player *n* folds                                |
/// | `PnWins(amount)`| Player *n* wins *amount* chips at showdown     |
/// | `PnLoses(amount)`| Player *n* loses *amount* chips at showdown  |
///
/// `DealCommon(pile)` records a community card deal (flop, turn, or river).
///
/// ## Serialization
///
/// Unit variants (`PnCheck`, `PnFold`) serialize as plain YAML strings.
/// All other variants serialize as single-entry YAML mappings.
/// [`BasicPile`] values are encoded as Unicode card strings, e.g. `"A♠ K♥"`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum Action {
    /// Deal community cards (flop, turn, or river). Serialized as `DealCommon: "9♣ 6♦ 5♥"`.
    DealCommon(BasicPile),
    /// Hole cards dealt to player 0. Serialized as `P0Dealt: "A♠ K♠"`.
    P0Dealt(BasicPile),
    /// Player 0 checks.
    P0Check,
    /// Player 0 calls, bets, or raises. Serialized as `P0CBR: <amount>`.
    P0CBR(usize),
    /// Player 0 folds.
    P0Fold,
    /// Player 0 wins chips. Serialized as `P0Wins: <amount>`.
    P0Wins(usize),
    /// Player 0 loses chips. Serialized as `P0Loses: <amount>`.
    P0Loses(usize),
    /// Hole cards dealt to player 1.
    P1Dealt(BasicPile),
    /// Player 1 checks.
    P1Check,
    /// Player 1 calls, bets, or raises.
    P1CBR(usize),
    /// Player 1 folds.
    P1Fold,
    /// Player 1 wins chips.
    P1Wins(usize),
    /// Player 1 loses chips.
    P1Loses(usize),
    /// Hole cards dealt to player 2.
    P2Dealt(BasicPile),
    /// Player 2 checks.
    P2Check,
    /// Player 2 calls, bets, or raises.
    P2CBR(usize),
    /// Player 2 folds.
    P2Fold,
    /// Player 2 wins chips.
    P2Wins(usize),
    /// Player 2 loses chips.
    P2Loses(usize),
    /// Hole cards dealt to player 3.
    P3Dealt(BasicPile),
    /// Player 3 checks.
    P3Check,
    /// Player 3 calls, bets, or raises.
    P3CBR(usize),
    /// Player 3 folds.
    P3Fold,
    /// Player 3 wins chips.
    P3Wins(usize),
    /// Player 3 loses chips.
    P3Loses(usize),
    /// Hole cards dealt to player 4.
    P4Dealt(BasicPile),
    /// Player 4 checks.
    P4Check,
    /// Player 4 calls, bets, or raises.
    P4CBR(usize),
    /// Player 4 folds.
    P4Fold,
    /// Player 4 wins chips.
    P4Wins(usize),
    /// Player 4 loses chips.
    P4Loses(usize),
    /// Hole cards dealt to player 5.
    P5Dealt(BasicPile),
    /// Player 5 checks.
    P5Check,
    /// Player 5 calls, bets, or raises.
    P5CBR(usize),
    /// Player 5 folds.
    P5Fold,
    /// Player 5 wins chips.
    P5Wins(usize),
    /// Player 5 loses chips.
    P5Loses(usize),
    /// Hole cards dealt to player 6.
    P6Dealt(BasicPile),
    /// Player 6 checks.
    P6Check,
    /// Player 6 calls, bets, or raises.
    P6CBR(usize),
    /// Player 6 folds.
    P6Fold,
    /// Player 6 wins chips.
    P6Wins(usize),
    /// Player 6 loses chips.
    P6Loses(usize),
    /// Hole cards dealt to player 7.
    P7Dealt(BasicPile),
    /// Player 7 checks.
    P7Check,
    /// Player 7 calls, bets, or raises.
    P7CBR(usize),
    /// Player 7 folds.
    P7Fold,
    /// Player 7 wins chips.
    P7Wins(usize),
    /// Player 7 loses chips.
    P7Loses(usize),
    /// Hole cards dealt to player 8.
    P8Dealt(BasicPile),
    /// Player 8 checks.
    P8Check,
    /// Player 8 calls, bets, or raises.
    P8CBR(usize),
    /// Player 8 folds.
    P8Fold,
    /// Player 8 wins chips.
    P8Wins(usize),
    /// Player 8 loses chips.
    P8Loses(usize),
    /// Hole cards dealt to player 9.
    P9Dealt(BasicPile),
    /// Player 9 checks.
    P9Check,
    /// Player 9 calls, bets, or raises.
    P9CBR(usize),
    /// Player 9 folds.
    P9Fold,
    /// Player 9 wins chips.
    P9Wins(usize),
    /// Player 9 loses chips.
    P9Loses(usize),
    /// Hole cards dealt to player 10.
    P10Dealt(BasicPile),
    /// Player 10 checks.
    P10Check,
    /// Player 10 calls, bets, or raises.
    P10CBR(usize),
    /// Player 10 folds.
    P10Fold,
    /// Player 10 wins chips.
    P10Wins(usize),
    /// Player 10 loses chips.
    P10Loses(usize),
    /// Hole cards dealt to player 11.
    P11Dealt(BasicPile),
    /// Player 11 checks.
    P11Check,
    /// Player 11 calls, bets, or raises.
    P11CBR(usize),
    /// Player 11 folds.
    P11Fold,
    /// Player 11 wins chips.
    P11Wins(usize),
    /// Player 11 loses chips.
    P11Loses(usize),
}

impl Serialize for Action {
    #[allow(clippy::too_many_lines)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        match self {
            Action::DealCommon(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("DealCommon", &pile.to_string())?;
                map.end()
            }
            Action::P0Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P0Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P0Check => serializer.serialize_str("P0Check"),
            Action::P0CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P0CBR", amount)?;
                map.end()
            }
            Action::P0Fold => serializer.serialize_str("P0Fold"),
            Action::P0Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P0Wins", amount)?;
                map.end()
            }
            Action::P0Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P0Loses", amount)?;
                map.end()
            }
            Action::P1Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P1Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P1Check => serializer.serialize_str("P1Check"),
            Action::P1CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P1CBR", amount)?;
                map.end()
            }
            Action::P1Fold => serializer.serialize_str("P1Fold"),
            Action::P1Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P1Wins", amount)?;
                map.end()
            }
            Action::P1Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P1Loses", amount)?;
                map.end()
            }
            Action::P2Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P2Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P2Check => serializer.serialize_str("P2Check"),
            Action::P2CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P2CBR", amount)?;
                map.end()
            }
            Action::P2Fold => serializer.serialize_str("P2Fold"),
            Action::P2Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P2Wins", amount)?;
                map.end()
            }
            Action::P2Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P2Loses", amount)?;
                map.end()
            }
            Action::P3Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P3Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P3Check => serializer.serialize_str("P3Check"),
            Action::P3CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P3CBR", amount)?;
                map.end()
            }
            Action::P3Fold => serializer.serialize_str("P3Fold"),
            Action::P3Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P3Wins", amount)?;
                map.end()
            }
            Action::P3Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P3Loses", amount)?;
                map.end()
            }
            Action::P4Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P4Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P4Check => serializer.serialize_str("P4Check"),
            Action::P4CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P4CBR", amount)?;
                map.end()
            }
            Action::P4Fold => serializer.serialize_str("P4Fold"),
            Action::P4Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P4Wins", amount)?;
                map.end()
            }
            Action::P4Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P4Loses", amount)?;
                map.end()
            }
            Action::P5Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P5Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P5Check => serializer.serialize_str("P5Check"),
            Action::P5CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P5CBR", amount)?;
                map.end()
            }
            Action::P5Fold => serializer.serialize_str("P5Fold"),
            Action::P5Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P5Wins", amount)?;
                map.end()
            }
            Action::P5Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P5Loses", amount)?;
                map.end()
            }
            Action::P6Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P6Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P6Check => serializer.serialize_str("P6Check"),
            Action::P6CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P6CBR", amount)?;
                map.end()
            }
            Action::P6Fold => serializer.serialize_str("P6Fold"),
            Action::P6Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P6Wins", amount)?;
                map.end()
            }
            Action::P6Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P6Loses", amount)?;
                map.end()
            }
            Action::P7Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P7Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P7Check => serializer.serialize_str("P7Check"),
            Action::P7CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P7CBR", amount)?;
                map.end()
            }
            Action::P7Fold => serializer.serialize_str("P7Fold"),
            Action::P7Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P7Wins", amount)?;
                map.end()
            }
            Action::P7Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P7Loses", amount)?;
                map.end()
            }
            Action::P8Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P8Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P8Check => serializer.serialize_str("P8Check"),
            Action::P8CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P8CBR", amount)?;
                map.end()
            }
            Action::P8Fold => serializer.serialize_str("P8Fold"),
            Action::P8Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P8Wins", amount)?;
                map.end()
            }
            Action::P8Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P8Loses", amount)?;
                map.end()
            }
            Action::P9Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P9Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P9Check => serializer.serialize_str("P9Check"),
            Action::P9CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P9CBR", amount)?;
                map.end()
            }
            Action::P9Fold => serializer.serialize_str("P9Fold"),
            Action::P9Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P9Wins", amount)?;
                map.end()
            }
            Action::P9Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P9Loses", amount)?;
                map.end()
            }
            Action::P10Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P10Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P10Check => serializer.serialize_str("P10Check"),
            Action::P10CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P10CBR", amount)?;
                map.end()
            }
            Action::P10Fold => serializer.serialize_str("P10Fold"),
            Action::P10Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P10Wins", amount)?;
                map.end()
            }
            Action::P10Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P10Loses", amount)?;
                map.end()
            }
            Action::P11Dealt(pile) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P11Dealt", &pile.to_string())?;
                map.end()
            }
            Action::P11Check => serializer.serialize_str("P11Check"),
            Action::P11CBR(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P11CBR", amount)?;
                map.end()
            }
            Action::P11Fold => serializer.serialize_str("P11Fold"),
            Action::P11Wins(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P11Wins", amount)?;
                map.end()
            }
            Action::P11Loses(amount) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("P11Loses", amount)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionVisitor;

        impl<'de> Visitor<'de> for ActionVisitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid Action variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<Action, E>
            where
                E: de::Error,
            {
                match value {
                    "P0Check" => Ok(Action::P0Check),
                    "P0Fold" => Ok(Action::P0Fold),
                    "P1Check" => Ok(Action::P1Check),
                    "P1Fold" => Ok(Action::P1Fold),
                    "P2Check" => Ok(Action::P2Check),
                    "P2Fold" => Ok(Action::P2Fold),
                    "P3Check" => Ok(Action::P3Check),
                    "P3Fold" => Ok(Action::P3Fold),
                    "P4Check" => Ok(Action::P4Check),
                    "P4Fold" => Ok(Action::P4Fold),
                    "P5Check" => Ok(Action::P5Check),
                    "P5Fold" => Ok(Action::P5Fold),
                    "P6Check" => Ok(Action::P6Check),
                    "P6Fold" => Ok(Action::P6Fold),
                    "P7Check" => Ok(Action::P7Check),
                    "P7Fold" => Ok(Action::P7Fold),
                    "P8Check" => Ok(Action::P8Check),
                    "P8Fold" => Ok(Action::P8Fold),
                    "P9Check" => Ok(Action::P9Check),
                    "P9Fold" => Ok(Action::P9Fold),
                    "P10Check" => Ok(Action::P10Check),
                    "P10Fold" => Ok(Action::P10Fold),
                    "P11Check" => Ok(Action::P11Check),
                    "P11Fold" => Ok(Action::P11Fold),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["P0Check", "P0Fold", "..."],
                    )),
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<Action, M::Error>
            where
                M: MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .ok_or_else(|| de::Error::missing_field("action type"))?;

                match key.as_str() {
                    "DealCommon" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::DealCommon(pile))
                    }
                    "P0Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P0Dealt(pile))
                    }
                    "P0CBR" => Ok(Action::P0CBR(map.next_value()?)),
                    "P0Wins" => Ok(Action::P0Wins(map.next_value()?)),
                    "P0Loses" => Ok(Action::P0Loses(map.next_value()?)),
                    "P1Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P1Dealt(pile))
                    }
                    "P1CBR" => Ok(Action::P1CBR(map.next_value()?)),
                    "P1Wins" => Ok(Action::P1Wins(map.next_value()?)),
                    "P1Loses" => Ok(Action::P1Loses(map.next_value()?)),
                    "P2Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P2Dealt(pile))
                    }
                    "P2CBR" => Ok(Action::P2CBR(map.next_value()?)),
                    "P2Wins" => Ok(Action::P2Wins(map.next_value()?)),
                    "P2Loses" => Ok(Action::P2Loses(map.next_value()?)),
                    "P3Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P3Dealt(pile))
                    }
                    "P3CBR" => Ok(Action::P3CBR(map.next_value()?)),
                    "P3Wins" => Ok(Action::P3Wins(map.next_value()?)),
                    "P3Loses" => Ok(Action::P3Loses(map.next_value()?)),
                    "P4Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P4Dealt(pile))
                    }
                    "P4CBR" => Ok(Action::P4CBR(map.next_value()?)),
                    "P4Wins" => Ok(Action::P4Wins(map.next_value()?)),
                    "P4Loses" => Ok(Action::P4Loses(map.next_value()?)),
                    "P5Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P5Dealt(pile))
                    }
                    "P5CBR" => Ok(Action::P5CBR(map.next_value()?)),
                    "P5Wins" => Ok(Action::P5Wins(map.next_value()?)),
                    "P5Loses" => Ok(Action::P5Loses(map.next_value()?)),
                    "P6Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P6Dealt(pile))
                    }
                    "P6CBR" => Ok(Action::P6CBR(map.next_value()?)),
                    "P6Wins" => Ok(Action::P6Wins(map.next_value()?)),
                    "P6Loses" => Ok(Action::P6Loses(map.next_value()?)),
                    "P7Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P7Dealt(pile))
                    }
                    "P7CBR" => Ok(Action::P7CBR(map.next_value()?)),
                    "P7Wins" => Ok(Action::P7Wins(map.next_value()?)),
                    "P7Loses" => Ok(Action::P7Loses(map.next_value()?)),
                    "P8Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P8Dealt(pile))
                    }
                    "P8CBR" => Ok(Action::P8CBR(map.next_value()?)),
                    "P8Wins" => Ok(Action::P8Wins(map.next_value()?)),
                    "P8Loses" => Ok(Action::P8Loses(map.next_value()?)),
                    "P9Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P9Dealt(pile))
                    }
                    "P9CBR" => Ok(Action::P9CBR(map.next_value()?)),
                    "P9Wins" => Ok(Action::P9Wins(map.next_value()?)),
                    "P9Loses" => Ok(Action::P9Loses(map.next_value()?)),
                    "P10Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P10Dealt(pile))
                    }
                    "P10CBR" => Ok(Action::P10CBR(map.next_value()?)),
                    "P10Wins" => Ok(Action::P10Wins(map.next_value()?)),
                    "P10Loses" => Ok(Action::P10Loses(map.next_value()?)),
                    "P11Dealt" => {
                        let pile_str: String = map.next_value()?;
                        let pile = pile_str
                            .parse::<Pile<Standard52>>()
                            .map_err(de::Error::custom)?
                            .into_basic_pile();
                        Ok(Action::P11Dealt(pile))
                    }
                    "P11CBR" => Ok(Action::P11CBR(map.next_value()?)),
                    "P11Wins" => Ok(Action::P11Wins(map.next_value()?)),
                    "P11Loses" => Ok(Action::P11Loses(map.next_value()?)),
                    _ => Err(de::Error::unknown_field(
                        &key,
                        &["DealCommon", "P0Dealt", "P0CBR", "..."],
                    )),
                }
            }
        }

        deserializer.deserialize_any(ActionVisitor)
    }
}

impl Action {
    /// Returns the seat number associated with this action, or [`None`] for [`Action::DealCommon`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use pkstate::act::Action;
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(Action::P3Check.get_seat_number(), Some(3));
    /// assert_eq!(Action::P11Fold.get_seat_number(), Some(11));
    /// assert_eq!(Action::DealCommon(basic!("A♠ K♠")).get_seat_number(), None);
    /// ```
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn get_seat_number(&self) -> Option<usize> {
        match self {
            Action::P0Dealt(_)
            | Action::P0Check
            | Action::P0CBR(_)
            | Action::P0Fold
            | Action::P0Wins(_)
            | Action::P0Loses(_) => Some(0),
            Action::P1Dealt(_)
            | Action::P1Check
            | Action::P1CBR(_)
            | Action::P1Fold
            | Action::P1Wins(_)
            | Action::P1Loses(_) => Some(1),
            Action::P2Dealt(_)
            | Action::P2Check
            | Action::P2CBR(_)
            | Action::P2Fold
            | Action::P2Wins(_)
            | Action::P2Loses(_) => Some(2),
            Action::P3Dealt(_)
            | Action::P3Check
            | Action::P3CBR(_)
            | Action::P3Fold
            | Action::P3Wins(_)
            | Action::P3Loses(_) => Some(3),
            Action::P4Dealt(_)
            | Action::P4Check
            | Action::P4CBR(_)
            | Action::P4Fold
            | Action::P4Wins(_)
            | Action::P4Loses(_) => Some(4),
            Action::P5Dealt(_)
            | Action::P5Check
            | Action::P5CBR(_)
            | Action::P5Fold
            | Action::P5Wins(_)
            | Action::P5Loses(_) => Some(5),
            Action::P6Dealt(_)
            | Action::P6Check
            | Action::P6CBR(_)
            | Action::P6Fold
            | Action::P6Wins(_)
            | Action::P6Loses(_) => Some(6),
            Action::P7Dealt(_)
            | Action::P7Check
            | Action::P7CBR(_)
            | Action::P7Fold
            | Action::P7Wins(_)
            | Action::P7Loses(_) => Some(7),
            Action::P8Dealt(_)
            | Action::P8Check
            | Action::P8CBR(_)
            | Action::P8Fold
            | Action::P8Wins(_)
            | Action::P8Loses(_) => Some(8),
            Action::P9Dealt(_)
            | Action::P9Check
            | Action::P9CBR(_)
            | Action::P9Fold
            | Action::P9Wins(_)
            | Action::P9Loses(_) => Some(9),
            Action::P10Dealt(_)
            | Action::P10Check
            | Action::P10CBR(_)
            | Action::P10Fold
            | Action::P10Wins(_)
            | Action::P10Loses(_) => Some(10),
            Action::P11Dealt(_)
            | Action::P11Check
            | Action::P11CBR(_)
            | Action::P11Fold
            | Action::P11Wins(_)
            | Action::P11Loses(_) => Some(11),
            Action::DealCommon(_) => None,
        }
    }
}

/// An ordered sequence of [`Action`]s representing one betting street.
///
/// A complete hand is modelled as a [`Vec<Round>`](Vec) inside [`PKState`](crate::PKState),
/// where each `Round` typically corresponds to one street: preflop, flop, turn, and river.
///
/// # Example
///
/// ```rust
/// use pkstate::act::{Action, Round};
/// use cardpack::prelude::*;
///
/// let flop = Round(vec![
///     Action::DealCommon(basic!("9♣ 6♦ 5♥")),
///     Action::P3Check,
///     Action::P4CBR(8000),
///     Action::P3CBR(26000),
///     Action::P4CBR(26000),
/// ]);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Round(pub Vec<Action>);
