#![warn(clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

use crate::game::{ForcedBets, GameType};
use crate::seat::Seat;
use crate::util::{deserialize_basic_pile_opt, serialize_basic_pile_opt};
use cardpack::prelude::BasicPile;
use serde::{Deserialize, Serialize};

pub mod game;
pub mod seat;
pub mod util;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PKState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub game: GameType,
    pub forced_bets: ForcedBets,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_basic_pile_opt",
        deserialize_with = "deserialize_basic_pile_opt",
        default
    )]
    pub board: Option<BasicPile>,
    pub players: Vec<Seat>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use cardpack::prelude::*;

    #[test]
    fn test_pkstate_yaml_serialization_empty() {
        let pkstate = PKState {
            id: None,
            game: GameType::NoLimitHoldem,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        println!("Empty PKState YAML:\n{}", yaml_string);

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
    fn test_pkstate_yaml_deserialization_all_game_types() {
        let game_types = vec![
            GameType::NoLimitHoldem,
            GameType::LimitHoldem,
            GameType::PLO,
            GameType::Razz,
        ];

        for game_type in game_types {
            let pkstate = PKState {
                id: None,
                game: game_type,
                forced_bets: ForcedBets::new(50, 100),
                board: None,
                players: vec![],
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
    fn test_seat_yaml_serialization() {
        let pile = basic!("2♠ 8♣");

        let seat = Seat {
            id: Some("player1".to_string()),
            name: "Alice".to_string(),
            hand: pile,
            stack: 1000,
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&seat).expect("Failed to serialize Seat to YAML");

        println!("Seat YAML:\n{}", yaml_string);

        // Verify the hand is serialized as a string
        assert!(
            yaml_string.contains("hand:"),
            "YAML should contain hand field"
        );

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
    fn test_seat_yaml_serialization_none_id() {
        let pile = BasicPile::default();

        let seat = Seat {
            id: None,
            name: "Bob".to_string(),
            hand: pile,
            stack: 500,
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&seat).expect("Failed to serialize Seat to YAML");

        println!("Seat without ID YAML:\n{}", yaml_string);

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
    fn test_pkstate_with_players_yaml_serialization() {
        let hand1 = basic!("2♠ 8♣");
        let hand2 = basic!("3♠ 6♣");

        let players = vec![
            Seat {
                id: Some("player1".to_string()),
                name: "Alice".to_string(),
                hand: hand1,
                stack: 1000,
            },
            Seat {
                id: Some("player2".to_string()),
                name: "Bob".to_string(),
                hand: hand2,
                stack: 500,
            },
        ];

        let pkstate = PKState {
            id: Some("game1".to_string()),
            game: GameType::NoLimitHoldem,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players,
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        println!("PKState with players YAML:\n{}", yaml_string);

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
    fn test_basic_pile_to_string() {
        let pile = BasicPile::default();

        // Convert to string representation
        let pile_str = pile.to_string();
        println!("BasicPile string representation: '{}'", pile_str);

        // Empty piles have empty string representation which is valid
        // Verify serialization of the pile works
        let yaml_str = serde_yaml_bw::to_string(&pile).expect("Failed to serialize BasicPile");
        println!("BasicPile YAML: {}", yaml_str);
    }

    #[test]
    fn test_pkstate_with_board_yaml_serialization() {
        let board = BasicPile::default();

        let pkstate = PKState {
            id: Some("game2".to_string()),
            game: GameType::NoLimitHoldem,
            forced_bets: ForcedBets::new(25, 50),
            board: Some(board),
            players: vec![],
        };

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstate)
            .expect("Failed to serialize PKState with board to YAML");

        println!("PKState with board YAML:\n{}", yaml_string);

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
    fn test_pkstate_option_none_skipped_in_yaml() {
        let pkstate = PKState {
            id: None,
            game: GameType::NoLimitHoldem,
            forced_bets: ForcedBets::new(50, 100),
            board: None,
            players: vec![],
        };

        // Serialize to YAML
        let yaml_string =
            serde_yaml_bw::to_string(&pkstate).expect("Failed to serialize PKState to YAML");

        println!("PKState with None fields YAML:\n{}", yaml_string);

        // Verify that Option::None fields are skipped
        assert!(
            !yaml_string.contains("id:"),
            "YAML should skip id field when None"
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
}
