use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;

use near_sdk::{log, PromiseOrValue};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_id = env::predecessor_account_id();
        let asset = self.internal_unwrap_asset(&token_id);

        assert!(
            asset.config.can_deposit,
            "Deposits for this asset are not enabled"
        );

        let amount = amount.0 * 10u128.pow(asset.config.extra_decimals as u32);
        log!(
            "===> sender_id: {:?}, deposit amount: {:?}, msg: {:?}",
            sender_id,
            amount,
            msg
        );

        PromiseOrValue::Value(U128(0))
    }
}
