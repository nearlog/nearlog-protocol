mod asset;
mod asset_config;
mod fungible_token;
mod pool;

pub use asset::*;
pub use asset_config::*;
pub use fungible_token::*;
pub use pool::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Timestamp};

pub(crate) type TokenId = AccountId;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Assets,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub assets: LookupMap<TokenId, Asset>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given config. Needs to be called once.
    #[init]
    pub fn new() -> Self {
        Self {
            assets: LookupMap::new(StorageKey::Assets),
        }
    }
}
