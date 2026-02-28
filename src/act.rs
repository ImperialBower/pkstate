use cardpack::prelude::{BasicPile, Pile, Standard52};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor, MapAccess};
use std::fmt;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum Action {
    DealCommon(BasicPile),
    P0Dealt(BasicPile),
    P0Check,
    P0CBR(usize),
    P0Fold,
    P0Wins(usize),
    P0Loses(usize),
    P1Dealt(BasicPile),
    P1Check,
    P1CBR(usize),
    P1Fold,
    P1Wins(usize),
    P1Loses(usize),
    P2Dealt(BasicPile),
    P2Check,
    P2CBR(usize),
    P2Fold,
    P2Wins(usize),
    P2Loses(usize),
    P3Dealt(BasicPile),
    P3Check,
    P3CBR(usize),
    P3Fold,
    P3Wins(usize),
    P3Loses(usize),
    P4Dealt(BasicPile),
    P4Check,
    P4CBR(usize),
    P4Fold,
    P4Wins(usize),
    P4Loses(usize),
    P5Dealt(BasicPile),
    P5Check,
    P5CBR(usize),
    P5Fold,
    P5Wins(usize),
    P5Loses(usize),
    P6Dealt(BasicPile),
    P6Check,
    P6CBR(usize),
    P6Fold,
    P6Wins(usize),
    P6Loses(usize),
    P7Dealt(BasicPile),
    P7Check,
    P7CBR(usize),
    P7Fold,
    P7Wins(usize),
    P7Loses(usize),
    P8Dealt(BasicPile),
    P8Check,
    P8CBR(usize),
    P8Fold,
    P8Wins(usize),
    P8Loses(usize),
    P9Dealt(BasicPile),
    P9Check,
    P9CBR(usize),
    P9Fold,
    P9Wins(usize),
    P9Loses(usize),
    P10Dealt(BasicPile),
    P10Check,
    P10CBR(usize),
    P10Fold,
    P10Wins(usize),
    P10Loses(usize),
    P11Dealt(BasicPile),
    P11Check,
    P11CBR(usize),
    P11Fold,
    P11Wins(usize),
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
                    _ => Err(de::Error::unknown_variant(value, &["P0Check", "P0Fold", "..."])),
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<Action, M::Error>
            where
                M: MapAccess<'de>,
            {
                let key: String = map.next_key()?.ok_or_else(|| de::Error::missing_field("action type"))?;

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
                    _ => Err(de::Error::unknown_field(&key, &["DealCommon", "P0Dealt", "P0CBR", "..."])),
                }
            }
        }

        deserializer.deserialize_any(ActionVisitor)
    }
}

impl Action {
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
            Action::DealCommon(_) => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Round(pub Vec<Action>);
