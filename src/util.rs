use cardpack::prelude::{BasicPile, Pile, Standard52};
use serde::{Deserialize, Deserializer, Serializer};

/// Custom serializer to convert `BasicPile` to string
///
/// # Errors
///
/// This function will return an error if the `BasicPile` cannot be converted to a string,
/// which should not happen under normal circumstances since `BasicPile` implements `ToString`.
pub fn serialize_basic_pile<S>(pile: &BasicPile, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&pile.to_string())
}

/// Custom deserializer to convert string back to `BasicPile`
///
/// # Errors
///
/// This function will return an error if the string cannot be parsed into a `Pile<Standard52>`,
/// which can happen if the string is not in the correct format or contains invalid card representations.
pub fn deserialize_basic_pile<'de, D>(deserializer: D) -> Result<BasicPile, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let pile = s
        .parse::<Pile<Standard52>>()
        .map_err(serde::de::Error::custom)?;
    Ok(pile.into_basic_pile())
}

/// Custom serializer to convert `Option<BasicPile>` to string
///
/// # Errors
///
/// This function will return an error if the `BasicPile` cannot be converted to a string,
pub fn serialize_basic_pile_opt<S>(
    pile: &Option<BasicPile>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match pile {
        Some(p) => serializer.serialize_str(&p.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Custom deserializer to convert string back to` Option<BasicPile>`
///
/// # Errors
///
/// This function will return an error if the string cannot be parsed into a `Pile<Standard52>`,
pub fn deserialize_basic_pile_opt<'de, D>(deserializer: D) -> Result<Option<BasicPile>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => s
            .parse::<Pile<Standard52>>()
            .map(|p| p.into_basic_pile())
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
