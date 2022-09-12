mod account;
mod account_asset;
mod actions;
mod asset;
mod asset_config;
mod big_decimal;
mod config;
mod fungible_token;
mod pool;
mod storage;
mod storage_tracker;

pub use account::*;
pub use account_asset::*;
pub use actions::*;
pub use asset::*;
pub use asset_config::*;
pub use big_decimal::*;
pub use config::*;
pub use fungible_token::*;
pub use pool::*;
pub use storage::*;
pub use storage_tracker::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    Promise, Timestamp,
};

use std::collections::HashMap;

pub(crate) type TokenId = AccountId;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Assets,
    Config,
    Storage,
    Accounts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub assets: LookupMap<TokenId, Asset>,
    pub config: LazyOption<Config>,
    pub storage: LookupMap<AccountId, Storage>,
    pub accounts: UnorderedMap<AccountId, Account>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given config. Needs to be called once.
    #[init]
    pub fn new(config: Config) -> Self {
        Self {
            assets: LookupMap::new(StorageKey::Assets),
            config: LazyOption::new(StorageKey::Config, Some(&config)),
            storage: LookupMap::new(StorageKey::Storage),
            accounts: UnorderedMap::new(StorageKey::Accounts),
        }
    }
}
