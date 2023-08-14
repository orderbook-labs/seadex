use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    CreatePool {
        base_denom: String,
        quote_denom: String,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_rate: u64,
        label: String,
    },
    // Who can freeze pool when the pool happen some problems
    FreezePool {
        lottery: String,
    },
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}
