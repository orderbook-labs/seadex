use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use sei_cosmwasm::SeiMsg;

use crate::{error::ContractError, msg::InstantiateMsg, QuerierWrapper};

use super::{CONTRACT_NAME, CONTRACT_VERSION};

pub fn instantiate(
    deps: DepsMut<QuerierWrapper>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;

    let attrs = vec![
        attr("action", "instantiate"),
        attr("sender", sender.as_str()),
    ];

    Ok(Response::new().add_attributes(attrs))
}
