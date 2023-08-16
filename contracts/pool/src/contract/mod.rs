pub mod exec;
pub mod init;
pub mod query;
pub mod reply;
pub mod sudo;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult};
use sei_cosmwasm::{SeiMsg, SeiQueryWrapper};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::{QuerierWrapper, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:seadex-pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// REPLY IDs
const PLACE_ORDER_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<QuerierWrapper>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    init::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<QuerierWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    exec::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<QuerierWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query::query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<QuerierWrapper>,
    env: Env,
    reply: Reply,
) -> Result<Response<SeiMsg>, ContractError> {
    reply::reply(deps, env, reply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<SeiMsg>, StdError> {
    sudo::sudo(deps, env, msg)
}
