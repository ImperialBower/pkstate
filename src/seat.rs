//! Player seat representation.
//!
//! This module provides [`Seat`], which describes a single player at the table.

use serde::{Deserialize, Serialize};

/// A player seated at the poker table.
///
/// Each seat records an optional identifier, the player's display name, and their current
/// chip stack. The `id` field is omitted from YAML output when [`None`].
///
/// # Example
///
/// ```rust
/// use pkstate::seat::Seat;
///
/// let seat = Seat {
///     id: Some("player-42".to_string()),
///     name: "Gus Hansen".to_string(),
///     stack: 1_000_000,
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Seat {
    /// Optional unique identifier for the player (e.g. a UUID or username).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The player's display name.
    pub name: String,
    /// The player's current chip stack.
    pub stack: usize,
}
