#![warn(clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use crate::game::GameType;

pub mod game;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PKState {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    game: GameType
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkstate_yaml_serialization() {
        let pkstate = PKState {
            id: None,
            game: GameType::NoLimitHoldem,
        };

        // Serialize to YAML
        let yaml_string = serde_yaml_bw::to_string(&pkstate)
            .expect("Failed to serialize PKState to YAML");

        println!("{}", yaml_string);

        // Verify the YAML contains the expected content
        assert!(yaml_string.contains("NoLimitHoldem"), "YAML should contain game type");

        // Deserialize from YAML
        let deserialized: PKState = serde_yaml_bw::from_str(&yaml_string)
            .expect("Failed to deserialize PKState from YAML");

        // Verify the deserialized state matches the original
        assert_eq!(pkstate, deserialized, "Deserialized PKState should match original");
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
            let pkstate = PKState { id: None, game: game_type };

            // Serialize and deserialize
            let yaml_string = serde_yaml_bw::to_string(&pkstate)
                .expect("Failed to serialize PKState to YAML");
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

}
