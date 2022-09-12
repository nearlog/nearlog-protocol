use crate::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum OptionType {
    Call,
    Put,
}

impl Display for OptionType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OptionType::Call => write!(f, "Call"),
            OptionType::Put => write!(f, "Put"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OptionOrder {
    pub option_type: OptionType,
    // Option amount
    pub amount: Balance,
    // Strike price of the option
    pub strike: Balance,
    // Time Option exprired
    pub expiration: u64,
    pub premium: u32,
}
