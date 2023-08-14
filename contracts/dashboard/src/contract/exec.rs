use cosmwasm_std::{attr, to_binary, Addr, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};

use pool::msg::ExecuteMsg as PoolExecuteMsg;
use pool::msg::InstantiateMsg as PoolInstantiateMsg;

use crate::state::{PoolInfo, PENDING_POOL};
use crate::{
    msg::ExecuteMsg,
    state::{OWNER, STATE},
    ContractError,
};

use super::CREATE_LOTTERY_REPLY_ID;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        CreatePool {
            base_denom,
            quote_denom,
            tick_size,
            taker_fee_rate,
            maker_rebate_rate,
            label,
        } => create_pool(
            deps,
            &env,
            &info,
            &base_denom,
            &quote_denom,
            tick_size,
            taker_fee_rate,
            maker_rebate_rate,
            &label,
        ),

        FreezePool { lottery } => freeze_pool(deps, &env, &info, &lottery),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_pool(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    base_denom: &str,
    quote_denom: &str,
    tick_size: u64,
    taker_fee_rate: u64,
    maker_rebate_fee: u64,
    label: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let state = STATE.load(deps.storage)?;

    let init_pool_msg = PoolInstantiateMsg::new(
        base_denom,
        quote_denom,
        tick_size,
        taker_fee_rate,
        maker_rebate_fee,
    );

    let msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: state.pool_code_id,
        msg: to_binary(&init_pool_msg)?,
        funds: vec![],
        label: label.to_owned(),
    };

    let msg = SubMsg::reply_on_success(msg, CREATE_LOTTERY_REPLY_ID);
    let attrs = vec![attr("action", "create_pool"), attr("sender", sender)];

    let pool = PoolInfo {
        base_denom: base_denom.into(),
        quote_denom: quote_denom.into(),
        tick_size,
        taker_fee_rate,
        maker_rebate_fee,
        created_time: env.block.time,
        created_by: sender.clone(),
        contract_addr: Addr::unchecked(""), // update by reply
    };

    PENDING_POOL.save(deps.storage, &pool)?;

    Ok(Response::new().add_submessage(msg).add_attributes(attrs))
}

pub fn freeze_pool(
    deps: DepsMut,
    _env: &Env,
    info: &MessageInfo,
    pool: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg = PoolExecuteMsg::Freeze {};
    let msg = WasmMsg::Execute {
        contract_addr: pool.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    let attrs = vec![
        attr("action", "freeze_pool"),
        attr("sender", info.sender.as_str()),
    ];

    Ok(Response::new().add_message(msg).add_attributes(attrs))
}
