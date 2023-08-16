pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub price_denom: String,
    pub asset_denom: String,
    pub tick_size: u64,
    pub taker_fee_rate: u64,
    pub maker_rebate_fee: u64,
}

impl InstantiateMsg {
    pub fn new(
        price_denom: impl Into<String>,
        asset_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
    ) -> Self {
        Self {
            price_denom: price_denom.into(),
            asset_denom: asset_denom.into(),
            tick_size,
            taker_fee_rate,
            maker_rebate_fee,
        }
    }
}
