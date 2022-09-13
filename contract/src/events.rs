use crate::*;

pub mod emit {
    use super::*;
    use near_sdk::serde_json::json;

    #[derive(Serialize)]
    #[serde(crate = "near_sdk::serde")]
    struct OptionLog<'a> {
        pub option_id: &'a OptionId,
    }

    #[derive(Serialize)]
    #[serde(crate = "near_sdk::serde")]
    struct OptionCreatedLog<'a> {
        pub option_id: &'a OptionId,
        pub option_type: &'a OptionType,
        pub amount: &'a Balance,
        pub strike: &'a Balance,
        pub expiration: &'a u64,
        pub premium: &'a f64,
    }

    fn log_event<T: Serialize>(event: &str, data: T) {
        let event = json!({
            "standard": "nearlog",
            "version": "0.1.0",
            "event": event,
            "data": [data]
        });

        log!("EVENT_JSON:{}", event.to_string());
    }

    pub fn create(option_id: &OptionId, option_order: &OptionOrder) {
        log_event(
            "create",
            OptionCreatedLog {
                option_id: &option_id,
                option_type: &option_order.option_type,
                amount: &option_order.amount,
                strike: &option_order.strike,
                expiration: &option_order.expiration,
                premium: &option_order.premium,
            },
        );
    }

    pub fn exercise(option_id: &OptionId) {
        log_event(
            "exercise",
            OptionLog {
                option_id: &option_id,
            },
        );
    }
}
