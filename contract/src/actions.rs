use near_sdk::json_types::U128;

use crate::*;

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize))]
#[serde(crate = "near_sdk::serde")]
pub struct OptionOrderReq {
    pub option_id: OptionId,
    pub token_id: TokenId,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize))]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    Exercise(OptionOrderReq),
}

impl Contract {
    pub fn internal_execute(
        &mut self,
        account_id: &AccountId,
        account: &mut Account,
        actions: Vec<Action>,
        prices: Prices,
    ) {
        for action in actions {
            match action {
                Action::Exercise(option_order_req) => {
                    self.internal_exercise_order(&option_order_req, &prices);
                }
            }
        }
    }

    pub fn internal_exercise_order(&mut self, option_order_req: &OptionOrderReq, prices: &Prices) {
        let asset = self.internal_unwrap_asset(&option_order_req.token_id);
        let price = prices.get_unwrap(&option_order_req.token_id);
        let option_order = self
            .internal_get_order(&option_order_req.option_id)
            .expect("Can't get the option order");
        // TODO
    }

    pub fn internal_deposit(
        &mut self,
        account: &mut Account,
        token_id: &TokenId,
        amount: Balance,
    ) -> Shares {
        let mut asset = self.internal_unwrap_asset(token_id);
        let mut account_asset = account.internal_get_asset_or_default(token_id);

        let shares: Shares = asset.supplied.amount_to_shares(amount, false);

        account_asset.deposit_shares(shares);
        account.internal_set_asset(&token_id, account_asset);

        asset.supplied.deposit(shares, amount);
        self.internal_set_asset(token_id, asset);

        shares
    }
}

#[near_bindgen]
impl Contract {
    /// Executes a given list actions on behalf of the predecessor account.
    /// - Requires one yoctoNEAR.
    #[payable]
    pub fn execute(&mut self, actions: Vec<Action>) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let mut account = self.internal_unwrap_account(&account_id);
        self.internal_execute(&account_id, &mut account, actions, Prices::new());
        self.internal_set_account(&account_id, account);
    }
}
