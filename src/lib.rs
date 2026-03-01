//! # pkstate
//!
//! A library for representing, serializing, and deserializing the state of a poker hand.
//!
//! `pkstate` models everything needed to describe a hand in progress or a complete hand history:
//! the game type, forced bets, the players at the table, the community board cards, and the full
//! sequence of actions across every street. All types implement [`serde::Serialize`] and
//! [`serde::Deserialize`], with [`BasicPile`] card collections serialized as human-readable
//! Unicode card strings (e.g. `"A♠ K♥"`).
//!
//! ## Modules
//!
//! - [`act`] — [`act::Action`] enum and [`act::Round`] newtype.
//! - [`game`] — [`game::GameType`] enum and [`game::ForcedBets`] struct.
//! - [`seat`] — [`seat::Seat`] struct representing a player at the table.
//! - [`util`] — Shared serde helpers for [`BasicPile`] serialization.
//!
//! ## Example
//!
//! ```rust
//! use pkstate::{PKState, act::{Action, Round}, game::{ForcedBets, GameType}, seat::Seat};
//! use cardpack::prelude::*;
//!
//! let players = vec![
//!     Seat { id: None, name: "Alice".to_string(), stack: 1_000 },
//!     Seat { id: None, name: "Bob".to_string(),   stack: 1_000 },
//! ];
//!
//! let preflop = Round(vec![
//!     Action::P0Dealt(basic!("A♠ K♠")),
//!     Action::P1Dealt(basic!("7♦ 2♣")),
//!     Action::P0CBR(100),
//!     Action::P1Fold,
//!     Action::P0Wins(100),
//! ]);
//!
//! let state = PKState {
//!     id: Some("example-hand".to_string()),
//!     datetime: None,
//!     game: GameType::NoLimitHoldem,
//!     button: 0,
//!     forced_bets: ForcedBets::new(50, 100),
//!     board: None,
//!     players,
//!     rounds: vec![preflop],
//! };
//!
//! let yaml = serde_yaml_bw::to_string(&state).unwrap();
//! let restored: PKState = serde_yaml_bw::from_str(&yaml).unwrap();
//! assert_eq!(state, restored);
//! ```

#![warn(clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

use crate::act::Round;
use crate::game::{ForcedBets, GameType};
use crate::seat::Seat;
use crate::util::{deserialize_basic_pile_opt, serialize_basic_pile_opt};
use cardpack::prelude::BasicPile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod act;
pub mod game;
pub mod seat;
pub mod util;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PKStates(pub Vec<PKState>);

impl From<Vec<PKState>> for PKStates {
    fn from(states: Vec<PKState>) -> Self {
        PKStates(states)
    }
}

/// The complete state of a poker hand.
///
/// `PKState` captures every piece of information needed to represent or replay a hand:
/// which game is being played, who is sitting where, what the forced bets are, what cards
/// are on the board, and the full ordered sequence of actions across every betting round.
///
/// Optional fields (`id`, `datetime`, `board`) are omitted from YAML when [`None`], keeping
/// serialized output concise.
///
/// # Example
///
/// ```rust
/// use pkstate::{PKState, game::{ForcedBets, GameType}, seat::Seat};
///
/// let state = PKState {
///     id: Some("hand-001".to_string()),
///     datetime: None,
///     game: GameType::NoLimitHoldem,
///     button: 0,
///     forced_bets: ForcedBets::new(50, 100),
///     board: None,
///     players: vec![],
///     rounds: vec![],
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PKState {
    /// Optional unique identifier for the hand (e.g. a UUID or slug).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional UTC timestamp of when the hand was played.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub datetime: Option<DateTime<Utc>>,
    /// The poker variant being played.
    pub game: GameType,
    /// Seat index (0-based) of the dealer button.
    pub button: usize,
    /// The forced bets (blinds, straddles, ante) for this hand.
    pub forced_bets: ForcedBets,
    /// The community board cards. Serialized as a Unicode card string, e.g. `"9♣ 6♦ 5♥ 5♠ 8♠"`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_basic_pile_opt",
        deserialize_with = "deserialize_basic_pile_opt",
        default
    )]
    pub board: Option<BasicPile>,
    /// The players seated at the table, in seat order.
    pub players: Vec<Seat>,
    /// The betting rounds (preflop, flop, turn, river, …), each containing an ordered list of
    /// [`Action`](act::Action)s.
    pub rounds: Vec<Round>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::act::Action;
    use cardpack::prelude::*;

    #[test]
    fn the_hand() {
        let players = vec![
            Seat {
                id: None,
                name: "Doyle Brunson".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Eli Elezra".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Antonio Esfandari".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Gus Hansen".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Daniel Negreanu".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Cory Zeidman".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Barry Greenstein".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: None,
                name: "Amnon Filippi".to_string(),
                stack: 1_000_000,
            },
        ];

        let preflop = Round(vec![
            Action::P1CBR(50),
            Action::P2CBR(100),
            Action::P1Dealt(basic!("8♣ 3♥")),
            Action::P2Dealt(basic!("A♦ Q♣")),
            Action::P3Dealt(basic!("5♦ 5♣")),
            Action::P4Dealt(basic!("6♠ 6♥")),
            Action::P5Dealt(basic!("K♠ J♦")),
            Action::P6Dealt(basic!("4♦ 4♣")),
            Action::P7Dealt(basic!("7♣ 2♦")),
            Action::P0Dealt(basic!("T♠ 2♥")),
            Action::P3CBR(2100),
            Action::P4CBR(5000),
            Action::P5Fold,
            Action::P6Fold,
            Action::P7Fold,
            Action::P0Fold,
            Action::P1Fold,
            Action::P2Fold,
            Action::P3CBR(5000),
        ]);

        let flop = Round(vec![
            Action::DealCommon(basic!("9♣ 6♦ 5♥")),
            Action::P3Check,
            Action::P4CBR(8000),
            Action::P3CBR(26000),
            Action::P4CBR(26000),
        ]);

        let turn = Round(vec![
            Action::DealCommon(basic!("5♠")),
            Action::P3CBR(24000),
            Action::P4CBR(24000),
        ]);

        let river = Round(vec![
            Action::DealCommon(basic!("8♠")),
            Action::P3Check,
            Action::P4CBR(65000),
            Action::P3CBR(945000),
            Action::P4CBR(945000),
            Action::P3Wins(1000150),
            Action::P4Loses(1000000),
        ]);

        let rounds = vec![preflop, flop, turn, river];

        let pkstate = PKState {
            id: Some("the_hand".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players,
            rounds,
        };

        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        println!("{}", yaml_string);
    }

    #[test]
    fn yaml_serialization_empty() {
        let pkstate = PKState {
            id: None,
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
            rounds: vec![],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        // Verify the YAML contains the expected content
        assert!(
            yaml_string.contains("NoLimitHoldem"),
            "YAML should contain game type"
        );

        // Deserialize from YAML
        let deserialized: PKState =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKState from YAML");

        // Verify the deserialized state matches the original
        assert_eq!(
            pkstate, deserialized,
            "Deserialized PKState should match original"
        );
    }

    #[test]
    fn yaml_deserialization_all_game_types() {
        let game_types = vec![
            GameType::NoLimitHoldem,
            GameType::LimitHoldem,
            GameType::PLO,
            GameType::Razz,
        ];

        for game_type in game_types {
            let pkstate = PKState {
                id: None,
                datetime: None,
                game: game_type,
                button: 0,
                forced_bets: ForcedBets::new(50, 100),
                board: None,
                players: vec![],
                rounds: vec![],
            };

            // Serialize and deserialize
            let yaml_string =
                serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");
            let deserialized: PKState = serde_yaml_bw::from_str(&yaml_string)
                .expect("Failed to deserialize PKState from YAML");

            // Verify round-trip preserves the game type
            assert_eq!(
                pkstate.game, deserialized.game,
                "Game type should be preserved in YAML round-trip for {:?}",
                game_type
            );
        }
    }

    #[test]
    fn yaml_serialization() {
        let seat = Seat {
            id: Some("player1".to_string()),
            name: "Alice".to_string(),
            stack: 1000,
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&seat).expect("Failed to serialize Seat to YAML");

        // Deserialize from YAML
        let deserialized: Seat =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize Seat from YAML");

        // Verify the deserialized seat matches the original
        assert_eq!(
            seat, deserialized,
            "Deserialized Seat should match original"
        );
        assert_eq!(deserialized.name, "Alice");
        assert_eq!(deserialized.stack, 1000);
    }

    #[test]
    fn yaml_serialization_none_id() {
        let seat = Seat {
            id: None,
            name: "Bob".to_string(),
            stack: 500,
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&seat).expect("Failed to serialize Seat to YAML");

        // Verify that the id field is not included (skipped) in YAML
        assert!(
            !yaml_string.contains("id:"),
            "YAML should skip the id field when None"
        );

        // Deserialize from YAML
        let deserialized: Seat =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize Seat from YAML");

        // Verify the deserialized seat matches the original
        assert_eq!(
            seat, deserialized,
            "Deserialized Seat should match original"
        );
        assert_eq!(deserialized.id, None);
    }

    #[test]
    fn with_players_yaml_serialization() {
        let players = vec![
            Seat {
                id: Some("player1".to_string()),
                name: "Alice".to_string(),
                stack: 1000,
            },
            Seat {
                id: Some("player2".to_string()),
                name: "Bob".to_string(),
                stack: 500,
            },
        ];

        let pkstate = PKState {
            id: Some("game1".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players,
            rounds: vec![],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        // Verify the YAML structure
        assert!(
            yaml_string.contains("id: game1"),
            "YAML should contain game id"
        );
        assert!(
            yaml_string.contains("players:"),
            "YAML should contain players array"
        );
        assert!(
            yaml_string.contains("Alice"),
            "YAML should contain player name"
        );
        assert!(
            yaml_string.contains("Bob"),
            "YAML should contain player name"
        );

        // Deserialize from YAML
        let deserialized: PKState =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKState from YAML");

        // Verify the deserialized state matches the original
        assert_eq!(
            pkstate, deserialized,
            "Deserialized PKState should match original"
        );
        assert_eq!(deserialized.players.len(), 2);
        assert_eq!(deserialized.players[0].name, "Alice");
        assert_eq!(deserialized.players[1].name, "Bob");
    }

    #[test]
    fn basic_pile_to_string() {
        let pile = BasicPile::default();

        // Empty piles have empty string representation which is valid
        // Verify serialization of the pile works
        let yaml_str = serde_yaml_bw::to_string(&pile).expect("Failed to serialize BasicPile");
        println!("BasicPile YAML: {}", yaml_str);
    }

    #[test]
    fn with_board_yaml_serialization() {
        let board = BasicPile::default();

        let pkstate = PKState {
            id: Some("game2".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(25, 50),
            board: Some(board),
            players: vec![],
            rounds: vec![],
        };

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstate)
            .expect("Failed to serialize PKState with board to YAML");

        // Verify the board is serialized as a string
        assert!(
            yaml_string.contains("board:"),
            "YAML should contain board field"
        );

        // Deserialize from YAML
        let deserialized: PKState =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKState from YAML");

        // Verify the deserialized state matches the original
        assert_eq!(
            pkstate, deserialized,
            "Deserialized PKState should match original"
        );
        assert!(deserialized.board.is_some(), "Board should be present");
    }

    #[test]
    fn option_none_skipped_in_yaml() {
        let pkstate = PKState {
            id: None,
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
            rounds: vec![
                Round(vec![Action::P2Check, Action::P0CBR(200), Action::P1Fold]),
                Round(vec![Action::P2Check, Action::P0CBR(200), Action::P1Fold]),
            ],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        // Verify that Option::None fields are skipped
        assert!(
            !yaml_string.contains("id:"),
            "YAML should skip id field when None"
        );
        assert!(
            !yaml_string.contains("datetime:"),
            "YAML should skip datetime field when None"
        );
        assert!(
            !yaml_string.contains("board:"),
            "YAML should skip board field when None"
        );

        // Verify that required fields are still present
        assert!(
            yaml_string.contains("game:"),
            "YAML should contain game field"
        );
        assert!(
            yaml_string.contains("forced_bets:"),
            "YAML should contain forced_bets field"
        );
        assert!(
            yaml_string.contains("players:"),
            "YAML should contain players field"
        );
    }

    #[test]
    fn with_datetime_yaml_serialization() {
        use chrono::TimeZone;

        let datetime = Utc.with_ymd_and_hms(2024, 3, 15, 10, 30, 0).unwrap();

        let pkstate = PKState {
            id: Some("game3".to_string()),
            datetime: Some(datetime),
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
            rounds: vec![],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        println!("PKState with datetime YAML:\n{}", yaml_string);

        // Verify the datetime is serialized
        assert!(
            yaml_string.contains("datetime:"),
            "YAML should contain datetime field"
        );

        // Deserialize from YAML
        let deserialized: PKState =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKState from YAML");

        // Verify the deserialized state matches the original
        assert_eq!(
            pkstate, deserialized,
            "Deserialized PKState should match original"
        );
        assert_eq!(
            deserialized.datetime,
            Some(datetime),
            "Datetime should be preserved"
        );
    }

    #[test]
    fn action_serialization_with_basic_pile() {
        // Test DealCommon with BasicPile
        let deal_common = Action::DealCommon(basic!("A♠ K♠ Q♠"));
        let yaml = serde_yaml_bw::to_string(&deal_common).expect("Failed to serialize DealCommon");
        println!("DealCommon YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize DealCommon");
        assert_eq!(deal_common, deserialized, "DealCommon should round-trip");

        // Test P0Dealt with BasicPile
        let p0_dealt = Action::P0Dealt(basic!("A♥ K♥"));
        let yaml = serde_yaml_bw::to_string(&p0_dealt).expect("Failed to serialize P0Dealt");
        println!("P0Dealt YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P0Dealt");
        assert_eq!(p0_dealt, deserialized, "P0Dealt should round-trip");

        // Test P5Dealt with BasicPile
        let p5_dealt = Action::P5Dealt(basic!("7♦ 2♣"));
        let yaml = serde_yaml_bw::to_string(&p5_dealt).expect("Failed to serialize P5Dealt");
        println!("P5Dealt YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P5Dealt");
        assert_eq!(p5_dealt, deserialized, "P5Dealt should round-trip");

        // Test actions without BasicPile
        let p0_check = Action::P0Check;
        let yaml = serde_yaml_bw::to_string(&p0_check).expect("Failed to serialize P0Check");
        println!("P0Check YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P0Check");
        assert_eq!(p0_check, deserialized, "P0Check should round-trip");

        let p2_cbr = Action::P2CBR(500);
        let yaml = serde_yaml_bw::to_string(&p2_cbr).expect("Failed to serialize P2CBR");
        println!("P2CBR YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P2CBR");
        assert_eq!(p2_cbr, deserialized, "P2CBR should round-trip");

        let p3_wins = Action::P3Wins(1000);
        let yaml = serde_yaml_bw::to_string(&p3_wins).expect("Failed to serialize P3Wins");
        println!("P3Wins YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P3Wins");
        assert_eq!(p3_wins, deserialized, "P3Wins should round-trip");

        let p7_fold = Action::P7Fold;
        let yaml = serde_yaml_bw::to_string(&p7_fold).expect("Failed to serialize P7Fold");
        println!("P7Fold YAML:\n{}", yaml);
        let deserialized: Action =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize P7Fold");
        assert_eq!(p7_fold, deserialized, "P7Fold should round-trip");
    }

    #[test]
    fn round_serialization_with_dealt_actions() {
        let round = Round(vec![
            Action::P0Dealt(basic!("A♠ A♥")),
            Action::P1Dealt(basic!("K♦ K♣")),
            Action::P2Dealt(basic!("Q♠ Q♥")),
            Action::DealCommon(basic!("J♠ T♠ 9♠")),
            Action::P0CBR(100),
            Action::P1CBR(300),
            Action::P2Fold,
            Action::P0CBR(600),
            Action::P1CBR(600),
        ]);

        let yaml =
            serde_yaml_bw::to_string(&round).expect("Failed to serialize Round with dealt actions");
        println!("Round with dealt actions YAML:\n{}", yaml);

        let deserialized: Round =
            serde_yaml_bw::from_str(&yaml).expect("Failed to deserialize Round with dealt actions");
        assert_eq!(round, deserialized, "Round should round-trip correctly");
    }

    // ====== PKStates YAML Serialization Tests ======
    // These are AI generated tests. I am not a big fan, but they do the work.

    #[test]
    fn pkstates_empty_yaml_serialization() {
        let pkstates = PKStates(vec![]);

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstates).expect("Failed to serialize empty PKStates to YAML");

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize empty PKStates");

        // Verify the deserialized PKStates matches the original
        assert_eq!(pkstates, deserialized, "Empty PKStates should round-trip");
        assert_eq!(deserialized.0.len(), 0, "Deserialized PKStates should be empty");
    }

    #[test]
    fn pkstates_single_hand_yaml_serialization() {
        let state = PKState {
            id: Some("hand-001".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![
                Seat {
                    id: Some("p1".to_string()),
                    name: "Alice".to_string(),
                    stack: 1000,
                },
                Seat {
                    id: Some("p2".to_string()),
                    name: "Bob".to_string(),
                    stack: 2000,
                },
            ],
            rounds: vec![Round(vec![
                Action::P0Dealt(basic!("A♠ K♠")),
                Action::P1Dealt(basic!("7♦ 2♣")),
                Action::P0CBR(100),
                Action::P1Fold,
            ])],
        };

        let pkstates = PKStates(vec![state.clone()]);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize single-hand PKStates to YAML");

        // Verify YAML structure
        assert!(
            yaml_string.contains("- id:"),
            "YAML should contain hand id"
        );
        assert!(
            yaml_string.contains("NoLimitHoldem"),
            "YAML should contain game type"
        );
        assert!(
            yaml_string.contains("Alice"),
            "YAML should contain player name"
        );

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify round-trip
        assert_eq!(pkstates, deserialized, "Single-hand PKStates should round-trip");
        assert_eq!(deserialized.0.len(), 1, "Deserialized PKStates should have 1 hand");
        assert_eq!(
            deserialized.0[0].id,
            Some("hand-001".to_string()),
            "Hand id should be preserved"
        );
        assert_eq!(
            deserialized.0[0].players.len(),
            2,
            "Hand should have 2 players"
        );
    }

    #[test]
    fn pkstates_multiple_hands_yaml_serialization() {
        let state1 = PKState {
            id: Some("hand-001".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![
                Seat {
                    id: None,
                    name: "Alice".to_string(),
                    stack: 1000,
                },
                Seat {
                    id: None,
                    name: "Bob".to_string(),
                    stack: 2000,
                },
            ],
            rounds: vec![],
        };

        let state2 = PKState {
            id: Some("hand-002".to_string()),
            datetime: None,
            game: GameType::LimitHoldem,
            button: 1,
            forced_bets: ForcedBets::new(25, 50),
            board: Some(basic!("9♣ 6♦ 5♥")),
            players: vec![
                Seat {
                    id: None,
                    name: "Charlie".to_string(),
                    stack: 5000,
                },
                Seat {
                    id: None,
                    name: "Diana".to_string(),
                    stack: 3000,
                },
                Seat {
                    id: None,
                    name: "Eve".to_string(),
                    stack: 4000,
                },
            ],
            rounds: vec![Round(vec![Action::P0Check, Action::P1CBR(50)])],
        };

        let pkstates = PKStates(vec![state1.clone(), state2.clone()]);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize multi-hand PKStates to YAML");

        println!("Multi-hand PKStates YAML:\n{}", yaml_string);

        // Verify YAML structure
        assert!(
            yaml_string.contains("hand-001"),
            "YAML should contain first hand id"
        );
        assert!(
            yaml_string.contains("hand-002"),
            "YAML should contain second hand id"
        );
        assert!(
            yaml_string.contains("NoLimitHoldem"),
            "YAML should contain NoLimitHoldem game type"
        );
        assert!(
            yaml_string.contains("LimitHoldem"),
            "YAML should contain LimitHoldem game type"
        );
        assert!(
            yaml_string.contains("board:"),
            "YAML should contain board for hand 2"
        );

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify round-trip
        assert_eq!(
            pkstates, deserialized,
            "Multi-hand PKStates should round-trip"
        );
        assert_eq!(
            deserialized.0.len(),
            2,
            "Deserialized PKStates should have 2 hands"
        );

        // Verify first hand
        assert_eq!(
            deserialized.0[0].id,
            Some("hand-001".to_string()),
            "First hand id should be preserved"
        );
        assert_eq!(
            deserialized.0[0].game,
            GameType::NoLimitHoldem,
            "First hand game type should be preserved"
        );
        assert_eq!(
            deserialized.0[0].button,
            0,
            "First hand button should be preserved"
        );
        assert_eq!(
            deserialized.0[0].players.len(),
            2,
            "First hand should have 2 players"
        );

        // Verify second hand
        assert_eq!(
            deserialized.0[1].id,
            Some("hand-002".to_string()),
            "Second hand id should be preserved"
        );
        assert_eq!(
            deserialized.0[1].game,
            GameType::LimitHoldem,
            "Second hand game type should be preserved"
        );
        assert_eq!(
            deserialized.0[1].button,
            1,
            "Second hand button should be preserved"
        );
        assert_eq!(
            deserialized.0[1].board,
            Some(basic!("9♣ 6♦ 5♥")),
            "Second hand board should be preserved"
        );
        assert_eq!(
            deserialized.0[1].players.len(),
            3,
            "Second hand should have 3 players"
        );
        assert_eq!(
            deserialized.0[1].rounds.len(),
            1,
            "Second hand should have 1 round"
        );
    }

    #[test]
    fn pkstates_with_datetime_yaml_serialization() {
        use chrono::TimeZone;

        let datetime1 = Utc.with_ymd_and_hms(2024, 3, 15, 10, 30, 0).unwrap();
        let datetime2 = Utc.with_ymd_and_hms(2024, 3, 15, 12, 45, 30).unwrap();

        let state1 = PKState {
            id: Some("hand-001".to_string()),
            datetime: Some(datetime1),
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
            rounds: vec![],
        };

        let state2 = PKState {
            id: Some("hand-002".to_string()),
            datetime: Some(datetime2),
            game: GameType::PLO,
            button: 1,
            forced_bets: ForcedBets::new(25, 50),
            board: None,
            players: vec![],
            rounds: vec![],
        };

        let pkstates = PKStates(vec![state1, state2]);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize PKStates with datetimes to YAML");

        println!("PKStates with datetimes YAML:\n{}", yaml_string);

        // Verify datetime is serialized
        assert!(
            yaml_string.contains("datetime:"),
            "YAML should contain datetime fields"
        );

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify round-trip preserves datetimes
        assert_eq!(
            deserialized.0[0].datetime,
            Some(datetime1),
            "First hand datetime should be preserved"
        );
        assert_eq!(
            deserialized.0[1].datetime,
            Some(datetime2),
            "Second hand datetime should be preserved"
        );
    }

    #[test]
    fn pkstates_with_all_game_types_yaml_serialization() {
        let game_types = vec![
            GameType::NoLimitHoldem,
            GameType::LimitHoldem,
            GameType::PLO,
            GameType::Razz,
        ];

        let mut pkstates_vec = Vec::new();

        for (idx, game_type) in game_types.iter().enumerate() {
            let state = PKState {
                id: Some(format!("hand-{:03}", idx)),
                datetime: None,
                game: *game_type,
                button: idx % 2,
                forced_bets: ForcedBets::new(50, 100),
                board: None,
                players: vec![],
                rounds: vec![],
            };
            pkstates_vec.push(state);
        }

        let pkstates = PKStates(pkstates_vec);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize PKStates with all game types to YAML");

        // Verify all game types are serialized
        assert!(
            yaml_string.contains("NoLimitHoldem"),
            "YAML should contain NoLimitHoldem"
        );
        assert!(
            yaml_string.contains("LimitHoldem"),
            "YAML should contain LimitHoldem"
        );
        assert!(yaml_string.contains("PLO"), "YAML should contain PLO");
        assert!(yaml_string.contains("Razz"), "YAML should contain Razz");

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify round-trip preserves all game types
        assert_eq!(deserialized.0.len(), 4, "Should have 4 hands");
        assert_eq!(deserialized.0[0].game, GameType::NoLimitHoldem);
        assert_eq!(deserialized.0[1].game, GameType::LimitHoldem);
        assert_eq!(deserialized.0[2].game, GameType::PLO);
        assert_eq!(deserialized.0[3].game, GameType::Razz);
    }

    #[test]
    fn pkstates_with_complex_actions_yaml_serialization() {
        let round1 = Round(vec![
            Action::P0Dealt(basic!("A♠ K♠")),
            Action::P1Dealt(basic!("Q♦ J♦")),
            Action::P2Dealt(basic!("T♥ 9♥")),
            Action::P0CBR(100),
            Action::P1CBR(300),
            Action::P2Fold,
            Action::P0CBR(600),
            Action::P1CBR(600),
        ]);

        let round2 = Round(vec![
            Action::DealCommon(basic!("K♣ 9♣ 5♠")),
            Action::P0Check,
            Action::P1CBR(500),
            Action::P0CBR(1500),
            Action::P1Fold,
            Action::P0Wins(2500),
        ]);

        let state = PKState {
            id: Some("complex-hand".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![
                Seat {
                    id: Some("alice".to_string()),
                    name: "Alice".to_string(),
                    stack: 10000,
                },
                Seat {
                    id: Some("bob".to_string()),
                    name: "Bob".to_string(),
                    stack: 8000,
                },
                Seat {
                    id: Some("charlie".to_string()),
                    name: "Charlie".to_string(),
                    stack: 5000,
                },
            ],
            rounds: vec![round1, round2],
        };

        let pkstates = PKStates(vec![state.clone()]);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize PKStates with complex actions to YAML");

        println!("Complex PKStates YAML:\n{}", yaml_string);

        // Verify structure
        assert!(
            yaml_string.contains("P0Dealt"),
            "YAML should contain dealt actions"
        );
        assert!(
            yaml_string.contains("P0CBR") || yaml_string.contains("cbr:"),
            "YAML should contain bet/raise actions"
        );
        assert!(
            yaml_string.contains("P1Fold") || yaml_string.contains("fold:"),
            "YAML should contain fold actions"
        );
        assert!(
            yaml_string.contains("DealCommon"),
            "YAML should contain common card actions"
        );

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize complex PKStates");

        // Verify round-trip
        assert_eq!(pkstates, deserialized, "Complex PKStates should round-trip");
        assert_eq!(
            deserialized.0[0].rounds.len(),
            2,
            "Hand should have 2 rounds"
        );
        assert_eq!(
            deserialized.0[0].players.len(),
            3,
            "Hand should have 3 players"
        );
    }

    #[test]
    fn pkstates_with_varied_stacks_yaml_serialization() {
        let state = PKState {
            id: Some("varied-stacks".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 2,
            forced_bets: ForcedBets::new(10, 20),
            board: None,
            players: vec![
                Seat {
                    id: None,
                    name: "Short Stack".to_string(),
                    stack: 100,
                },
                Seat {
                    id: None,
                    name: "Medium Stack".to_string(),
                    stack: 5000,
                },
                Seat {
                    id: None,
                    name: "Big Stack".to_string(),
                    stack: 50000,
                },
                Seat {
                    id: None,
                    name: "All In".to_string(),
                    stack: 0,
                },
            ],
            rounds: vec![],
        };

        let pkstates = PKStates(vec![state]);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize PKStates with varied stacks to YAML");

        // Verify stacks are preserved
        assert!(
            yaml_string.contains("100"),
            "YAML should contain short stack"
        );
        assert!(
            yaml_string.contains("5000"),
            "YAML should contain medium stack"
        );
        assert!(
            yaml_string.contains("50000"),
            "YAML should contain big stack"
        );
        assert!(
            yaml_string.contains("0"),
            "YAML should contain all-in stack"
        );

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify stacks are preserved
        assert_eq!(
            deserialized.0[0].players[0].stack,
            100,
            "Short stack should be preserved"
        );
        assert_eq!(
            deserialized.0[0].players[1].stack,
            5000,
            "Medium stack should be preserved"
        );
        assert_eq!(
            deserialized.0[0].players[2].stack,
            50000,
            "Big stack should be preserved"
        );
        assert_eq!(
            deserialized.0[0].players[3].stack,
            0,
            "All-in stack should be preserved"
        );
    }

    #[test]
    fn pkstates_yaml_serialization_preserves_order() {
        let mut states = Vec::new();

        for i in 0..5 {
            let state = PKState {
                id: Some(format!("hand-{}", i)),
                datetime: None,
                game: GameType::NoLimitHoldem,
                button: i,
                forced_bets: ForcedBets::new(50, 100),
                board: None,
                players: vec![
                    Seat {
                        id: None,
                        name: format!("Player {}", i * 2),
                        stack: 1000 + (i * 100),
                    },
                    Seat {
                        id: None,
                        name: format!("Player {}", i * 2 + 1),
                        stack: 2000 - (i * 50),
                    },
                ],
                rounds: vec![],
            };
            states.push(state);
        }

        let pkstates = PKStates(states);

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstates)
            .expect("Failed to serialize PKStates to YAML");

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify order is preserved
        assert_eq!(deserialized.0.len(), 5, "Should have 5 hands");

        for i in 0..5 {
            assert_eq!(
                deserialized.0[i].id,
                Some(format!("hand-{}", i)),
                "Hand {} id should be preserved in order",
                i
            );
            assert_eq!(
                deserialized.0[i].button,
                i,
                "Hand {} button should be preserved in order",
                i
            );
            assert_eq!(
                deserialized.0[i].players[0].name,
                format!("Player {}", i * 2),
                "Hand {} first player name should be preserved in order",
                i
            );
        }
    }

    #[test]
    fn pkstates_from_vec_conversion() {
        let states = vec![
            PKState {
                id: Some("hand-1".to_string()),
                datetime: None,
                game: GameType::NoLimitHoldem,
                button: 0,
                forced_bets: ForcedBets::new(50, 100),
                board: None,
                players: vec![],
                rounds: vec![],
            },
            PKState {
                id: Some("hand-2".to_string()),
                datetime: None,
                game: GameType::LimitHoldem,
                button: 1,
                forced_bets: ForcedBets::new(25, 50),
                board: None,
                players: vec![],
                rounds: vec![],
            },
        ];

        let pkstates: PKStates = states.into();

        // Verify conversion works
        assert_eq!(pkstates.0.len(), 2, "PKStates should have 2 hands");
        assert_eq!(
            pkstates.0[0].id,
            Some("hand-1".to_string()),
            "First hand should be preserved"
        );
        assert_eq!(
            pkstates.0[1].id,
            Some("hand-2".to_string()),
            "Second hand should be preserved"
        );

        // Verify it can be serialized/deserialized
        let yaml_string =
            serde_yaml_bw::to_string(&pkstates).expect("Failed to serialize PKStates to YAML");
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");
        assert_eq!(pkstates, deserialized, "Converted PKStates should round-trip");
    }

    #[test]
    fn pkstates_partial_none_fields_yaml_serialization() {
        let state1 = PKState {
            id: Some("hand-1".to_string()),
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
            rounds: vec![],
        };

        let state2 = PKState {
            id: None,
            datetime: None,
            game: GameType::NoLimitHoldem,
            button: 0,
            forced_bets: ForcedBets::new(50, 100),
            board: Some(basic!("A♠ K♠ Q♠")),
            players: vec![],
            rounds: vec![],
        };

        let pkstates = PKStates(vec![state1, state2]);

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstates).expect("Failed to serialize PKStates to YAML");

        println!("Partial None fields YAML:\n{}", yaml_string);

        // Deserialize from YAML
        let deserialized: PKStates =
            serde_yaml_bw::from_str(&yaml_string).expect("Failed to deserialize PKStates");

        // Verify first hand
        assert_eq!(
            deserialized.0[0].id,
            Some("hand-1".to_string()),
            "First hand id should be present"
        );
        assert_eq!(
            deserialized.0[0].board,
            None,
            "First hand board should be None"
        );

        // Verify second hand
        assert_eq!(
            deserialized.0[1].id,
            None,
            "Second hand id should be None"
        );
        assert_eq!(
            deserialized.0[1].board,
            Some(basic!("A♠ K♠ Q♠")),
            "Second hand board should be present"
        );
    }
}
