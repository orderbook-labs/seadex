use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub base_denom: String,
    pub quote_denom: String,
    pub tick_size: u64,
    pub taker_fee_rate: u64,
    pub maker_rebate_rate: u64,
    pub created_time: u64,
    pub created_by: Addr,
    pub next_bid_id: u64,
    pub next_ask_id: u64,
}

impl State {
    pub fn new(
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_rate: u64,
        created_time: u64,
        created_by: Addr,
    ) -> Self {
        Self {
            base_denom: base_denom.into(),
            quote_denom: quote_denom.into(),
            tick_size,
            taker_fee_rate,
            maker_rebate_rate,
            created_time,
            created_by,
            next_bid_id: 0,
            next_ask_id: 0,
        }
    }
}

pub struct Bid {}

pub struct Ask {}

pub struct Order {}

/// State Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");

pub const BIDS: Map<u128, Vec<Bid>> = Map::new("bid-orders");
pub const ASKS: Map<u128, Vec<Ask>> = Map::new("ask-orders");
pub const FILLED: Map<u128, Vec<Order>> = Map::new("filled-orders");
