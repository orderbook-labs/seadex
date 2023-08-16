use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response, SubMsg};
use cw_storage_plus::Item;
use sei_cosmwasm::{OrderType, PositionDirection, SeiMsg};

use crate::{
    auth::exec::{validate_owner, validate_position_effect},
    msg::ExecuteMsg,
    state::{State, OWNER, STATE},
    ContractError, OrderData, SeiOrder, SeiQueryWrapper,
};

use super::PLACE_ORDER_REPLY_ID;

pub fn execute(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    use ExecuteMsg::*;

    match msg {
        LimitAsk {} => limit_ask(deps, &env, &info),
        MarketAsk {} => market_ask(deps, &env, &info),
        LimitBid {
            price,
            quantity,
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
            leverage,
            &position_effect,
            &status_description,
            nominal,
            STATE,
        ),
        MarketBid {} => market_bid(deps, &env, &info),
        MakeMarket {} => make_market(deps, &env, &info),
        CancelOrders { order_ids } => cancel_order(deps, &env, &info, order_ids),
        SetDexContract { addr } => set_dex_contract_addr(deps, &info, &addr, STATE),
    }
}

fn limit_ask(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response<SeiMsg>, ContractError> {
    Ok(Response::default())
}

fn market_ask(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response<SeiMsg>, ContractError> {
    Ok(Response::default())
}

#[allow(clippy::too_many_arguments)]
fn limit_bid(
    deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    info: &MessageInfo,
    price: u128,
    quantity: u128,
    leverage: u128,
    position_effect: &str,
    status_description: &str,
    nominal: u128,
    state: Item<State>,
) -> Result<Response<SeiMsg>, ContractError> {
    validate_position_effect(position_effect)?;

    let state = state.load(deps.storage)?;
    let price_denom = &state.price_denom;
    let asset_denom = &state.asset_denom;

    let order_data = OrderData {
        leverage: Decimal::raw(leverage),
        position_effect: position_effect.parse()?,
    };

    let data = serde_json::to_string(&order_data).unwrap();

    let order = SeiOrder {
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
        .filter(|fund| fund.denom == state.price_denom)
        .cloned()
        .collect();

    let order_msg = SeiMsg::PlaceOrders {
        orders: vec![order],
        funds,
        contract_address: state.dex_contract_addr,
    };

    let sub_msg = SubMsg::reply_on_success(order_msg, PLACE_ORDER_REPLY_ID);

    let resp = Response::new().add_submessage(sub_msg);

    Ok(resp)
}

fn market_bid(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response<SeiMsg>, ContractError> {
    Ok(Response::default())
}

fn make_market(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    _info: &MessageInfo,
) -> Result<Response<SeiMsg>, ContractError> {
    Ok(Response::default())
}

fn cancel_order(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: &Env,
    _info: &MessageInfo,
    _order_ids: Vec<u64>,
) -> Result<Response<SeiMsg>, ContractError> {
    Ok(Response::default())
}

fn set_dex_contract_addr(
    deps: DepsMut<SeiQueryWrapper>,
    info: &MessageInfo,
    addr: &str,
    state: Item<State>,
) -> Result<Response<SeiMsg>, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    validate_owner(&owner, info)?;

    state.update(deps.storage, |mut s| -> Result<State, ContractError> {
        s.dex_contract_addr = deps.api.addr_validate(addr)?;
        Ok(s)
    })?;

    Ok(Response::default())
}
