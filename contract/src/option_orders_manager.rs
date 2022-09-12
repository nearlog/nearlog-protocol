use crate::*;

impl Contract {
    pub fn internal_set_order(&mut self, option_id: &OptionId, option_order: OptionOrder) {
        self.option_orders.insert(option_id, &option_order.into());
    }

    pub fn internal_get_order(&self, option_id: &OptionId) -> Option<OptionOrder> {
        self.option_orders.get(option_id)
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_order(&mut self, option_order: &OptionOrder) {
        assert_one_yocto();
        assert!(option_order.strike > 0, "Amount is too small");

        let next_option_id = self.current_option_id + 1;
        self.current_option_id = next_option_id;

        self.internal_set_order(&next_option_id, option_order.clone());
    }

    pub fn exercise_order(&self, option_id: &OptionId) {
        let option_order = self
            .internal_get_order(&option_id)
            .expect("Can't get the option order");

        assert!(
            option_order.expiration > env::block_timestamp(),
            "Option has expired"
        );

        // uint256 profit = payProfit(option);

        // uint currentPrice = uint(priceProvider.latestAnswer());
        // require(option.strike <= currentPrice, "Current price is too low");
        // profit = currentPrice.sub(option.strike).mul(option.amount).div(currentPrice);
        // pool.send(option.holder, profit);
        // unlockFunds(option);
    }

    // pub fn get_orders(&self, from_index: Option<u64>, limit: Option<u64>) -> Option<OptionOrder> {
    //     // unordered_map_pagination(&self.option_orders)
    //     let xxx = unordered_map_pagination(&self.option_orders, from_index, limit)
    //         .into_iter()
    //         .map(
    //             |(
    //                 option_id,
    //                 OptionOrder {
    //                     option_type,
    //                     amount,
    //                     strike,
    //                     period,
    //                     premium,
    //                 },
    //             )| {
    //                 OptionOrder {
    //                     option_type,
    //                     amount,
    //                     strike,
    //                     period,
    //                     premium,
    //                 }
    //             },
    //         )
    //         .collect();

    //     xxx
    // }
}
