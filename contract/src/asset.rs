use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct Asset {
    /// Total supplied, but excluding reserved.
    pub supplied: Pool,
    /// Total locked.
    pub locked: Pool,
    /// The amount reserved for the stability.
    pub reserved: Balance,
    /// When the asset was last updated. It's always going to be the current block timestamp.
    pub last_update_timestamp: Timestamp,
    /// The asset config.
    pub config: AssetConfig,
}

impl Asset {
    pub fn new(timestamp: Timestamp, config: AssetConfig) -> Self {
        Self {
            supplied: Pool::new(),
            locked: Pool::new(),
            reserved: 0,
            last_update_timestamp: timestamp,
            config,
        }
    }
}
