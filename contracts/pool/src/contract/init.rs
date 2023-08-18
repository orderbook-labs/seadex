use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use sei_cosmwasm::SeiMsg;

use crate::{
    error::ContractError,
    msg::InstantiateMsg,
    state::{State, ASK_ID, BID_ID, OWNER, STATE},
    SeiQueryWrapper,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION, SEI_DEX_CONTRACT_ADDR};

pub fn instantiate(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;
    let create_time = env.block.time;
    let dex_contract_addr = deps.api.addr_validate(SEI_DEX_CONTRACT_ADDR)?;

    let state = State::new(
        msg.price_denom,
        msg.asset_denom,
        msg.tick_size,
        msg.taker_fee_rate,
        msg.maker_rebate_fee,
        create_time,
        sender.clone(),
        dex_contract_addr,
    );

    STATE.save(deps.storage, &state)?;
    OWNER.save(deps.storage, sender)?;
    BID_ID.save(deps.storage, &1)?;
    ASK_ID.save(deps.storage, &1)?;

    let attrs = vec![
        attr("action", "instantiate"),
        attr("sender", sender.as_str()),
    ];

    Ok(Response::new().add_attributes(attrs))
}
