pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_denom: String,
    pub quote_denom: String,
    pub tick_size: u64,
    pub taker_fee_rate: u64,
    pub maker_rebate_rate: u64,
}

impl InstantiateMsg {
    pub fn new(
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_rate: u64,
    ) -> Self {
        Self {
            base_denom: base_denom.into(),
            quote_denom: quote_denom.into(),
            tick_size,
            taker_fee_rate,
            maker_rebate_rate,
        }
    }
}
