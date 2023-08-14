use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub name: String,
    pub height: u64,
    pub created_time: Timestamp,
    pub created_by: Addr,
    pub pool_code_id: u64,
    pub pools_count: u64,
}

impl State {
    pub fn new(
        name: String,
        height: u64,
        created_time: Timestamp,
        created_by: Addr,
        pool_code_id: u64,
    ) -> Self {
        Self {
            name,
            height,
            created_time,
            created_by,
            pool_code_id,
            pools_count: 0,
        }
    }
}

#[cw_serde]
pub struct PoolInfo {
    pub base_denom: String,
    pub quote_denom: String,
    pub tick_size: u64,
    pub taker_fee_rate: u64,
    pub maker_rebate_fee: u64,
    pub created_time: Timestamp,
    pub created_by: Addr,
    pub contract_addr: Addr,
}

/// State Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");
pub const POOLS: Map<&Addr, PoolInfo> = Map::new("pools"); // (lottery address, lottery info)
                                                           // pub const PLAYERS: Map<&Addr, PlayerInfo> = Map::new("players");    // (player address, playing info)

/// Cache pool info for initialize pool, and handle in reply function
pub const PENDING_POOL: Item<PoolInfo> = Item::new("pending_pool");
