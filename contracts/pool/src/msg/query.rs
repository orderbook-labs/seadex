use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::{state::State, Order};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LimitBidsResp)]
    LimitBids {},
    #[returns(BalancesResp)]
    Balances {},
    #[returns(OwnerResp)]
    Owner {},
    #[returns(CurrentStateResp)]
    CurrentState {},
}

#[cw_serde]
pub struct LimitBidsResp {
    pub bids: Vec<Order>,
}

#[cw_serde]
pub struct BalancesResp {
    pub amount: Coin,
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
pub struct CurrentStateResp {
    pub state: State,
}
