use crate::util::{deserialize_basic_pile, serialize_basic_pile};
use cardpack::prelude::BasicPile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Seat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(
        serialize_with = "serialize_basic_pile",
        deserialize_with = "deserialize_basic_pile"
    )]
    pub hand: BasicPile,
    pub stack: usize,
}
