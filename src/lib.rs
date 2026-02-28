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
pub struct PKState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub datetime: Option<DateTime<Utc>>,
    pub game: GameType,
    pub button: usize,
    pub forced_bets: ForcedBets,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_basic_pile_opt",
        deserialize_with = "deserialize_basic_pile_opt",
        default
    )]
    pub board: Option<BasicPile>,
    pub players: Vec<Seat>,
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
                id: Some("player0".to_string()),
                name: "Doyle Brunson".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player1".to_string()),
                name: "Eli Elezra".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player2".to_string()),
                name: "Antonio Esfandari".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player3".to_string()),
                name: "Gus Hansen".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player4".to_string()),
                name: "Daniel Negreanu".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player5".to_string()),
                name: "Cory Zeidman".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player6".to_string()),
                name: "Barry Greenstein".to_string(),
                stack: 1_000_000,
            },
            Seat {
                id: Some("player7".to_string()),
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
            Action::P3Wins(2000150),
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
}
