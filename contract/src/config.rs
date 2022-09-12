use crate::*;

/// Contract config
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// The account ID of the contract owner that allows to modify config, assets and use reserves.
    pub owner_id: AccountId,
}

impl Contract {
    pub fn internal_config(&self) -> Config {
        self.config.get().unwrap()
    }

    pub fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.internal_config().owner_id,
            "Not an owner"
        );
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_asset(&mut self, token_id: AccountId, asset_config: AssetConfig) {
        assert_one_yocto();
        self.assert_owner();
        self.internal_set_asset(&token_id, Asset::new(env::block_timestamp(), asset_config))
    }
}
