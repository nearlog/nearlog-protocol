use crate::*;

impl Contract {
    pub fn internal_set_order(&mut self, option_id: &OptionId, option_order: OptionOrder) {
        self.option_orders.insert(option_id, &option_order.into());
    }

    pub fn internal_get_order(&self, option_id: &OptionId) -> Option<OptionOrder> {
        self.option_orders.get(option_id)
    }

    pub fn internal_get_order_or_default(&mut self, option_id: &OptionId) -> OptionOrder {
        self.internal_get_order(option_id)
            .unwrap_or_else(OptionOrder::new)
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
}
