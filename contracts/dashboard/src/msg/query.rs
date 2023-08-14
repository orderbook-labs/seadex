use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::{PoolInfo, State};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PoolsResp)]
    Pools {},
    #[returns(BalancesResp)]
    Balances {},
    #[returns(OwnerResp)]
    Owner {},
    #[returns(CurrentStateResp)]
    CurrentState {},
}

#[cw_serde]
pub struct BalancesResp {
    pub amount: Coin,
}

#[cw_serde]
pub struct CurrentStateResp {
    pub state: State,
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
pub struct PoolsResp {
    pub lotteries: Vec<PoolInfo>,
}
