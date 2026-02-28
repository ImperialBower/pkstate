use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Seat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub stack: usize,
}
