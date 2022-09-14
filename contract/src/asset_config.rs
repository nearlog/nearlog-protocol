use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AssetConfig {
    /// The ratio of interest that is reserved by the protocol (multiplied by 10000).
    /// E.g. 2500 means 25% from interests goes to the reserve.
    pub reserve_ratio: u32,
    /// The maximum share of the pool size that
    /// could be utilized as a collateral in the options.
    /// Example: if `MaxUtilizationRate` = 50, then only 50%
    /// of liquidity on the pools contracts would be used for
    /// collateralizing options while 50% will be sitting idle
    /// available for withdrawals by the liquidity providers.
    /// The utilization ratio in a range of 50% — 100%
    pub max_utilization_rate: u32,

    /// The collateralization ratio for the option.
    /// collateral size that will be locked at the moment of buying them.
    /// The collateralization ratio in a range of 30% — 100%
    /// Example: if `CollateralizationRatio` = 50, then 50% of an option's
    /// notional size will be locked in the pools at the moment of buying it:
    /// say, 1 ETH call option will be collateralized with 0.5 ETH (50%).
    pub collateralization_ratio: u32,

    /// Volatility ratio (multiplied by 10000).
    pub volatility_ratio: u32,

    /// The risk free rate
    pub risk_free: u32,

    /// The amount of extra decimals to use for the fungible token. For example, if the asset like
    /// USDT has `6` decimals in the metadata, the `extra_decimals` can be set to `12`, to make the
    /// inner balance of USDT at `18` decimals.
    pub extra_decimals: u8,
    /// Whether the deposits of this assets are enabled.
    pub can_deposit: bool,
    /// Whether the withdrawals of this assets are enabled.
    pub can_withdraw: bool,
}
