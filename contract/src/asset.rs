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

impl Contract {
    pub fn internal_unwrap_asset(&self, token_id: &TokenId) -> Asset {
        self.internal_get_asset(token_id).expect("Asset not found")
    }

    pub fn internal_get_asset(&self, token_id: &TokenId) -> Option<Asset> {
        let asset = self.assets.get(token_id);
        asset
    }

    pub fn internal_set_asset(&mut self, token_id: &TokenId, asset: Asset) {
        self.assets.insert(token_id, &asset.into());
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_asset(&self, token_id: AccountId) -> Option<Asset> {
        self.internal_get_asset(&token_id)
    }
}
