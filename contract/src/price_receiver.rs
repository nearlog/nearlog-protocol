use crate::*;
use near_sdk::serde_json;

pub type AssetId = String;
pub type DurationSec = u32;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetOptionalPrice {
    pub asset_id: AssetId,
    pub price: Option<Price>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceData {
    pub timestamp: Timestamp,
    pub recency_duration_sec: DurationSec,

    pub prices: Vec<AssetOptionalPrice>,
}

pub trait OraclePriceReceiver {
    fn oracle_on_call(&mut self, sender_id: AccountId, data: PriceData, msg: String);
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize))]
#[serde(crate = "near_sdk::serde")]
pub enum PriceReceiverMsg {
    Execute { actions: Vec<Action> },
}

#[near_bindgen]
impl OraclePriceReceiver for Contract {
    /// The method will execute a given list of actions in the msg using the prices from the `data`
    /// provided by the oracle on behalf of the sender_id.
    /// - Requires to be called by the oracle account ID.
    fn oracle_on_call(&mut self, sender_id: AccountId, data: PriceData, msg: String) {
        assert_eq!(env::predecessor_account_id(), self.get_oracle_account_id());

        let actions = match serde_json::from_str(&msg).expect("Can't parse PriceReceiverMsg") {
            PriceReceiverMsg::Execute { actions } => actions,
        };

        let mut account = self.internal_unwrap_account(&sender_id);
        self.internal_execute(&sender_id, &mut account, actions, data.into());
        self.internal_set_account(&sender_id, account);
    }
}
