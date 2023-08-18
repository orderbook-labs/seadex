use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use sei_cosmwasm::GetOrdersResponse;

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
    #[returns(GetOrdersResponse)]
    GetOrders {
        contract_addr: String,
        account: String,
    },
    // #[returns(ExchangeRatesResp)]
    // ExchangeRates {},
    // OracleTwaps {
    //     lookback_seconds: u64,
    // },
    // DexTwaps {
    //     contract_address: String,
    //     lookback_seconds: u64,
    // },
    // OrderSimulation {
    //     order: SeiOrder,
    //     contract_address: String,
    // },
    // Epoch {},
    // GetOrderById {
    //     contract_address: String,
    //     price_denom: String,
    //     asset_denom: String,
    //     id: u64,
    // },
    // GetLatestPrice {
    //     contract_address: String,
    //     price_denom: String,
    //     asset_denom: String,
    // },
    // GetDenomAuthorityMetadata {
    //     denom: String,
    // },
    // GetDenomsFromCreator {
    //     creator: String,
    // },
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
