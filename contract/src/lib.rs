mod account;
mod account_asset;
mod actions;
mod asset;
mod asset_config;
mod big_decimal;
mod black_scholes;
mod config;
mod events;
mod fungible_token;
mod option_order;
mod option_orders_manager;
mod pool;
mod price_receiver;
mod prices;
mod storage;
mod storage_tracker;
mod utils;

pub use account::*;
pub use account_asset::*;
pub use actions::*;
pub use asset::*;
pub use asset_config::*;
pub use big_decimal::*;
pub use black_scholes::*;
pub use config::*;
pub use events::*;
pub use fungible_token::*;
pub use option_order::*;
pub use option_orders_manager::*;
pub use pool::*;
pub use price_receiver::*;
pub use prices::*;
pub use storage::*;
pub use storage_tracker::*;
pub use utils::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    Promise, Timestamp,
};

use std::collections::HashMap;

pub(crate) type TokenId = AccountId;
pub(crate) type OptionId = u32;
pub type TimestampSec = u32;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Assets,
    Config,
    Storage,
    Accounts,
    OptionOrders,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub assets: LookupMap<TokenId, Asset>,
    pub config: LazyOption<Config>,
    pub storage: LookupMap<AccountId, Storage>,
    pub accounts: UnorderedMap<AccountId, Account>,
    pub option_orders: UnorderedMap<OptionId, OptionOrder>,
    pub current_option_id: OptionId,
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
            option_orders: UnorderedMap::new(StorageKey::OptionOrders),
            current_option_id: 0u32,
        }
    }
}
