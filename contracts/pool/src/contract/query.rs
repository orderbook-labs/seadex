use cosmwasm_std::{to_binary, Binary, Coin, Deps, Env, Order as Ordering, StdResult};
use sei_cosmwasm::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, GetLatestPriceResponse,
    GetOrderByIdResponse, GetOrdersResponse, OracleTwapsResponse, OrderSimulationResponse,
    SeiQuerier,
};

use crate::{
    msg::{CurrentStateResp, LimitBidsResp, OwnerResp, QueryMsg},
    state::{BIDS, OWNER, STATE},
    SeiOrder, SeiQueryWrapper,
};

pub fn query(deps: Deps<SeiQueryWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LimitBids {} => to_binary(&limit_bids(deps)?),
        QueryMsg::Owner {} => owner(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::CurrentState {} => current_state(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::Balances {} => balances(deps, &env).and_then(|cs| to_binary(&cs)),
        QueryMsg::GetOrders {
            contract_addr,
            account,
        } => to_binary(&get_orders(deps, &contract_addr, &account)?),
        // QueryMsg::ExchangeRates {} => to_binary(&query_exchange_rates(deps)?),
        // QueryMsg::OracleTwaps { lookback_seconds } => {
        //     to_binary(&query_oracle_twaps(deps, lookback_seconds)?)
        // }
        // QueryMsg::DexTwaps {
        //     contract_address,
        //     lookback_seconds,
        // } => to_binary(&query_dex_twaps(deps, contract_address, lookback_seconds)?),
        // QueryMsg::OrderSimulation {
        //     order,
        //     contract_address,
        // } => to_binary(&query_order_simulation(deps, order, contract_address)?),
        // QueryMsg::Epoch {} => to_binary(&query_epoch(deps)?),
        // QueryMsg::GetOrderById {
        //     contract_address,
        //     price_denom,
        //     asset_denom,
        //     id,
        // } => to_binary(&query_get_order_by_id(
        //     deps,
        //     contract_address,
        //     price_denom,
        //     asset_denom,
        //     id,
        // )?),
        // QueryMsg::GetLatestPrice {
        //     contract_address,
        //     price_denom,
        //     asset_denom,
        // } => to_binary(&query_get_latest_price(
        //     deps,
        //     contract_address,
        //     price_denom,
        //     asset_denom,
        // )?),
    }
}

pub fn limit_bids(deps: Deps<SeiQueryWrapper>) -> StdResult<LimitBidsResp> {
    let bids: StdResult<Vec<_>> = BIDS
        .range(deps.storage, None, None, Ordering::Ascending)
        .collect();
    let bids = bids?.into_iter().flat_map(|(_, bids)| bids).collect();

    Ok(LimitBidsResp { bids })
}

pub fn owner(deps: Deps<SeiQueryWrapper>) -> StdResult<OwnerResp> {
    let owner = OWNER.load(deps.storage)?;
    Ok(OwnerResp { owner })
}

pub fn current_state(deps: Deps<SeiQueryWrapper>) -> StdResult<CurrentStateResp> {
    let state = STATE.load(deps.storage)?;
    Ok(CurrentStateResp { state })
}

pub fn balances(deps: Deps<SeiQueryWrapper>, env: &Env) -> StdResult<Vec<Coin>> {
    deps.querier.query_all_balances(&env.contract.address)
}

pub fn get_orders(
    deps: Deps<SeiQueryWrapper>,
    contract_address: &str,
    account: &str,
) -> StdResult<GetOrdersResponse> {
    let valid_addr = deps.api.addr_validate(contract_address)?;
    let valid_acc = deps.api.addr_validate(account)?;
    let querier = SeiQuerier::new(&deps.querier);

    querier.query_get_orders(valid_addr, valid_acc)
}

pub fn query_exchange_rates(deps: Deps<SeiQueryWrapper>) -> StdResult<ExchangeRatesResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates()?;

    Ok(res)
}

pub fn query_oracle_twaps(
    deps: Deps<SeiQueryWrapper>,
    lookback_seconds: u64,
) -> StdResult<OracleTwapsResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: OracleTwapsResponse = querier.query_oracle_twaps(lookback_seconds)?;

    Ok(res)
}

pub fn query_dex_twaps(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    lookback_seconds: u64,
) -> StdResult<DexTwapsResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: DexTwapsResponse = querier.query_dex_twaps(lookback_seconds, valid_addr)?;

    Ok(res)
}

pub fn query_order_simulation(
    deps: Deps<SeiQueryWrapper>,
    order: SeiOrder,
    contract_address: String,
) -> StdResult<OrderSimulationResponse> {
    let contract_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: OrderSimulationResponse = querier.query_order_simulation(order, contract_addr)?;

    Ok(res)
}

pub fn query_epoch(deps: Deps<SeiQueryWrapper>) -> StdResult<EpochResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: EpochResponse = querier.query_epoch()?;

    Ok(res)
}

pub fn get_order_by_id(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    price_denom: String,
    asset_denom: String,
    order_id: u64,
) -> StdResult<GetOrderByIdResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetOrderByIdResponse =
        querier.query_get_order_by_id(valid_addr, price_denom, asset_denom, order_id)?;

    Ok(res)
}

pub fn get_latest_price(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    price_denom: String,
    asset_denom: String,
) -> StdResult<GetLatestPriceResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetLatestPriceResponse =
        querier.query_get_latest_price(valid_addr, price_denom, asset_denom)?;

    Ok(res)
}

// pub fn query_denom_authority_metadata(
//     deps: Deps<SeiQueryWrapper>,
//     denom: String,
// ) -> StdResult<DenomAuthorityMetadataResponse> {
//     let querier = SeiQuerier::new(&deps.querier);
//     let res: DenomAuthorityMetadataResponse = querier.query_denom_authority_metadata(denom)?;

//     Ok(res)
// }

// pub fn query_denoms_from_creator(
//     deps: Deps<SeiQueryWrapper>,
//     creator: String,
// ) -> StdResult<DenomsFromCreatorResponse> {
//     let creator_addr = deps.api.addr_validate(&creator)?;
//     let querier = SeiQuerier::new(&deps.querier);
//     let res: DenomsFromCreatorResponse = querier.query_denoms_from_creator(creator_addr)?;

//     Ok(res)
// }
