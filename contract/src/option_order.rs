use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct OptionOrder {
    pub option_type: u8,
    pub amount: Balance,
    pub strike: Balance,
    pub period: u32,
    pub premium: u32,
}
