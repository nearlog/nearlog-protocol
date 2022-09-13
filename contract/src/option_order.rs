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
    // Option ID
    pub option_id: OptionId,
    // Option type
    pub option_type: OptionType,
    // Option amount
    pub amount: Balance,
    // Strike price of the option
    pub strike: Balance,
    // Time Option exprired
    pub expiration: u64,
    // Option premium is the income received by an investor who sells an option contract, or the current price of an option contract that has yet to expire.
    pub premium: f64,
}

impl OptionOrder {
    pub fn new() -> Self {
        Self {
            option_id: 0,
            option_type: OptionType::Call,
            amount: 0,
            strike: 0,
            expiration: 0,
            premium: 0.0,
        }
    }
}
