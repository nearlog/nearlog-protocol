use crate::*;
use near_sdk::json_types::U128;

pub type Shares = U128;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Pool {
    pub shares: Shares,
    pub balance: Balance,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            shares: 0.into(),
            balance: 0,
        }
    }
}
