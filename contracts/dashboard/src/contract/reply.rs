use cosmwasm_std::{attr, to_binary, DepsMut, Env, Reply, Response, StdError, SubMsgResponse};
use cw_utils::parse_instantiate_response_data;

use crate::{
    msg::InstantiationData,
    state::{PENDING_POOL, POOLS, STATE},
    ContractError,
};

use super::CREATE_LOTTERY_REPLY_ID;

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CREATE_LOTTERY_REPLY_ID => {
            initial_lottery_instantiated(deps, env, reply.result.into_result())
        }
        id => Err(ContractError::UnRecognizedReplyId { id }),
    }
}

pub fn initial_lottery_instantiated(
    deps: DepsMut,
    _env: Env,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    // Parse data from reply
    let resp = reply.map_err(StdError::generic_err)?;
    let data = resp.data.ok_or(ContractError::DataMissing {})?;
    let resp = parse_instantiate_response_data(&data)?;

    let pool_addr = &deps.api.addr_validate(&resp.contract_address)?;

    let mut pool = PENDING_POOL.load(deps.storage)?;
    pool.contract_addr = pool_addr.to_owned();

    POOLS.save(deps.storage, pool_addr, &pool)?;

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.pools_count += 1;
        Ok(state)
    })?;

    let attrs = vec![attr("action", "reply_create_lottery")];

    let data = InstantiationData {
        addr: pool_addr.to_owned(),
    };
    let data = to_binary(&data)?;

    Ok(Response::new().add_attributes(attrs).set_data(data))
}
