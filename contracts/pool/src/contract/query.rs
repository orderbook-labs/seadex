use cosmwasm_std::{to_binary, Binary, Coin, Deps, Env, Order, StdResult};

use crate::{
    msg::{CurrentStateResp, LimitBidsResp, OwnerResp, QueryMsg},
    state::{BIDS, OWNER, STATE},
    SeiQueryWrapper,
};

pub fn query(deps: Deps<SeiQueryWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LimitBids {} => to_binary(&limit_bids(deps)?),
        QueryMsg::Owner {} => owner(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::CurrentState {} => current_state(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::Balances {} => balances(deps, &env).and_then(|cs| to_binary(&cs)),
    }
}

pub fn limit_bids(deps: Deps<SeiQueryWrapper>) -> StdResult<LimitBidsResp> {
    let bids: StdResult<Vec<_>> = BIDS
        .range(deps.storage, None, None, Order::Ascending)
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
