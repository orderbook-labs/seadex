use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{msg::ExecuteMsg, ContractError};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        LimitAsk {} => limit_ask(deps, &env, &info),
        MarketAsk {} => market_ask(deps, &env, &info),
        LimitBid {} => limit_bid(deps, &env, &info),
        MarketBid {} => market_bid(deps, &env, &info),
        MakeMarket {} => make_market(deps, &env, &info),
        CancelOrder { order_id } => cancel_order(deps, &env, &info, order_id),
        CancelAllOrders {} => cancel_all_orders(deps, &env, &info),
        Freeze {} => freeze(deps, &env, &info),
    }
}

fn limit_ask(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn market_ask(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn limit_bid(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn market_bid(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn make_market(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn cancel_order(
    _deps: DepsMut,
    _env: &Env,
    _info: &MessageInfo,
    _order_id: u128,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn cancel_all_orders(
    _deps: DepsMut,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn freeze(_deps: DepsMut, _env: &Env, _info: &MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::default())
}
