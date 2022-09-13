use std::ops::{Div, Mul};

use crate::*;

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize))]
#[serde(crate = "near_sdk::serde")]
pub struct OptionOrderReq {
    pub option_id: OptionId,
    pub option_type: OptionType,
    pub amount: Balance,
    pub token_id: TokenId,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize))]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    Create(OptionOrderReq),
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
                Action::Create(option_order_req) => {
                    self.internal_create_order(&option_order_req, &prices);
                }
                Action::Exercise(option_order_req) => {
                    self.internal_exercise_order(&option_order_req, &prices);
                }
            }
        }
    }

    pub fn internal_create_order(&mut self, option_order_req: &OptionOrderReq, prices: &Prices) {
        let mut asset = self.internal_unwrap_asset(&option_order_req.token_id);
        let price = prices.get_unwrap(&option_order_req.token_id);
        let mut option_order = self.internal_get_order_or_default(&option_order_req.option_id);

        // let spot_price =
        //     BigDecimal::from_balance_price(option_order.amount, price, asset.config.extra_decimals);

        // TMP
        let option = BlackScholes {
            S: 100.00,        // spot price
            K: 110.00,        // strike price
            r: 0.016,         // risk-free rate 1.6%
            t: 0.08333333333, // 1 month until expiration
            v: 0.15,          // 15% volatility
        };

        println!("{:.4}", option.call_price());
        println!("{:.4}", option.put_price());

        option_order.option_type = option_order_req.option_type.clone();
        option_order.amount = option_order_req.amount;
        if option_order_req.option_type == OptionType::Call {
            option_order.premium = option.call_price();
        } else {
            option_order.premium = option.put_price();
        }

        // Locked liquidity

        let available_amount = asset.available_amount();
        let max_lock_shares = asset.locked.amount_to_shares(available_amount, false);

        let (locked_shares, amount) = asset_amount_to_shares(
            &asset.locked,
            max_lock_shares,
            &option_order_req.amount,
            true,
        );

        asset.locked.deposit(locked_shares, option_order_req.amount);

        self.internal_set_asset(&option_order_req.token_id, asset);

        events::emit::create(&option_order_req.option_id, &option_order);
    }

    pub fn internal_exercise_order(&mut self, option_order_req: &OptionOrderReq, prices: &Prices) {
        let asset = self.internal_unwrap_asset(&option_order_req.token_id);
        let price = prices.get_unwrap(&option_order_req.token_id);
        let option_order = self
            .internal_get_order(&option_order_req.option_id)
            .expect("Can't get the option order");

        let current_price =
            BigDecimal::from_balance_price(option_order.amount, price, asset.config.extra_decimals);

        let strike_price =
            BigDecimal::from_balance_price(option_order.strike, price, asset.config.extra_decimals);

        assert!(strike_price.le(&current_price), "Current price is too low");

        let profit = current_price
            .sub(strike_price)
            .mul(BigDecimal::from(option_order.amount))
            .div(current_price);

        log!("===> Exercise: {:?}", profit.to_string());

        events::emit::exercise(&option_order_req.option_id);
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

fn asset_amount_to_shares(
    pool: &Pool,
    available_shares: Shares,
    amount: &u128,
    inverse_round_direction: bool,
) -> (Shares, Balance) {
    let (shares, amount) = (
        pool.amount_to_shares(amount.clone(), !inverse_round_direction),
        amount,
    );

    (shares, *amount)
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
