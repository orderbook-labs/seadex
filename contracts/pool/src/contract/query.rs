use cosmwasm_std::{to_binary, Binary, Coin, Deps, Env, StdResult};

use crate::{
    msg::{CurrentStateResp, OwnerResp, QueryMsg},
    state::{OWNER, STATE},
    QuerierWrapper,
};

pub fn query(deps: Deps<QuerierWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => owner(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::CurrentState {} => current_state(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::Balances {} => balances(deps, &env).and_then(|cs| to_binary(&cs)),
    }
}

pub fn owner(deps: Deps<QuerierWrapper>) -> StdResult<OwnerResp> {
    let owner = OWNER.load(deps.storage)?;
    Ok(OwnerResp { owner })
}

pub fn current_state(deps: Deps<QuerierWrapper>) -> StdResult<CurrentStateResp> {
    let state = STATE.load(deps.storage)?;
    Ok(CurrentStateResp { state })
}

pub fn balances(deps: Deps<QuerierWrapper>, env: &Env) -> StdResult<Vec<Coin>> {
    deps.querier.query_all_balances(&env.contract.address)
}
