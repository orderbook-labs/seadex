use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response};
use sei_cosmwasm::{OrderType, PositionDirection, SeiMsg};

use crate::{
    auth::exec::validate_position_effect, msg::ExecuteMsg, ContractError, Order, OrderData,
    QuerierWrapper,
};

pub fn execute(
    deps: DepsMut<QuerierWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        LimitAsk {} => limit_ask(deps, &env, &info),
        MarketAsk {} => market_ask(deps, &env, &info),
        LimitBid {
            price,
            quantity,
            price_denom,
            asset_denom,
            leverage,
            position_effect,
            status_description,
            nominal,
        } => limit_bid(
            deps,
            &env,
            &info,
            price,
            quantity,
            &price_denom,
            &asset_denom,
            leverage,
            &position_effect,
            &status_description,
            nominal,
        ),
        MarketBid {} => market_bid(deps, &env, &info),
        MakeMarket {} => make_market(deps, &env, &info),
        CancelOrder { order_id } => cancel_order(deps, &env, &info, order_id),
        CancelAllOrders {} => cancel_all_orders(deps, &env, &info),
        Freeze {} => freeze(deps, &env, &info),
    }
}

fn limit_ask(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn market_ask(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn limit_bid(
    deps: DepsMut<QuerierWrapper>,
    env: &Env,
    info: &MessageInfo,
    price: u128,
    quantity: u128,
    price_denom: &str,
    asset_denom: &str,
    leverage: u128,
    position_effect: &str,
    status_description: &str,
    nominal: u128,
) -> Result<Response, ContractError> {
    validate_position_effect(&position_effect)?;

    let order_data = OrderData {
        leverage: Decimal::raw(leverage),
        position_effect: position_effect.parse()?,
    };

    let data = serde_json::to_string(&order_data).unwrap();

    let order = Order {
        price: Decimal::raw(price),
        quantity: Decimal::raw(quantity),
        price_denom: price_denom.to_owned(),
        asset_denom: asset_denom.to_owned(),
        order_type: OrderType::Limit,
        position_direction: PositionDirection::Long,
        data,
        status_description: status_description.to_owned(),
        nominal: Decimal::raw(nominal),
    };

    let funds = info
        .funds
        .iter()
        .filter(|fund| fund.denom == price_denom)
        .cloned()
        .collect();

    let order_msg = SeiMsg::PlaceOrders {
        orders: vec![order],
        funds,
        contract_address: deps
            .api
            .addr_validate("sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m")?,
    };

    Ok(Response::default())
}

fn market_bid(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn make_market(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn cancel_order(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
    _order_id: u128,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn cancel_all_orders(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn freeze(
    _deps: DepsMut<QuerierWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
