use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum Action {
    P0Check,
    P0CBR(usize),
    P0Fold,
    P1Check,
    P1CBR(usize),
    P1Fold,
    P2Check,
    P2CBR(usize),
    P2Fold,
    P3Check,
    P3CBR(usize),
    P3Fold,
    P4Check,
    P4CBR(usize),
    P4Fold,
    P5Check,
    P5CBR(usize),
    P5Fold,
    P6Check,
    P6CBR(usize),
    P6Fold,
    P7Check,
    P7CBR(usize),
    P7Fold,
    P8Check,
    P8CBR(usize),
    P8Fold,
    P9Check,
    P9CBR(usize),
    P9Fold,
    P10Check,
    P10CBR(usize),
    P10Fold,
    P11Check,
    P11CBR(usize),
    P11Fold,
}

#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Round(pub Vec<Action>);