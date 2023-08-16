use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};
use sei_cosmwasm::Order;

#[cw_serde]
pub struct State {
    pub price_denom: String,
    pub asset_denom: String,
    pub tick_size: u64,
    pub taker_fee_rate: u64,
    pub maker_rebate_fee: u64,
    pub created_time: Timestamp,
    pub created_by: Addr,
    pub next_bid_id: u64,
    pub next_ask_id: u64,
    pub dex_contract_addr: Addr,
}

impl State {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        price_denom: impl Into<String>,
        asset_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        created_time: Timestamp,
        created_by: Addr,
        dex_contract_addr: Addr,
    ) -> Self {
        Self {
            price_denom: price_denom.into(),
            asset_denom: asset_denom.into(),
            tick_size,
            taker_fee_rate,
            maker_rebate_fee,
            created_time,
            created_by,
            next_bid_id: 0,
            next_ask_id: 0,
            dex_contract_addr,
        }
    }
}

/// State Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");

pub const BIDS: Map<u128, Vec<Order>> = Map::new("bid-orders");
pub const ASKS: Map<u128, Vec<Order>> = Map::new("ask-orders");
pub const FILLED: Map<u128, Vec<Order>> = Map::new("filled-orders");
