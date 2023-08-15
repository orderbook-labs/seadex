use cosmwasm_std::{DepsMut, Env, Reply, Response, StdError, SubMsgResponse};
use protobuf::Message;
use sei_cosmwasm::{MsgPlaceOrdersResponse, SeiQueryWrapper};

use crate::ContractError;

use super::PLACE_ORDER_REPLY_ID;

pub fn reply(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    reply: Reply,
) -> Result<Response, ContractError> {
    match reply.id {
        PLACE_ORDER_REPLY_ID => Ok(handle_place_order_reply(deps, reply)?),
        id => Err(ContractError::UnRecognizedReplyId { id }),
    }
}

pub fn handle_place_order_reply(
    deps: DepsMut<SeiQueryWrapper>,
    reply: Reply,
) -> Result<Response, StdError> {
    let submsg_response: SubMsgResponse =
        reply.result.into_result().map_err(StdError::generic_err)?;

    match submsg_response.data {
        Some(response_data) => {
            let parsed_order_response: MsgPlaceOrdersResponse =
                Message::parse_from_bytes(response_data.as_slice()).map_err(|_| {
                    StdError::parse_err("MsgPlaceOrdersResponse", "failed to parse data")
                })?;
            deps.api.debug(&format!(
                "Order results from contract {:?}",
                parsed_order_response
            ));

            Ok(Response::new()
                .add_attribute("method", "handle_place_order_reply")
                .add_attribute(
                    "order_ids",
                    format!("{:?}", parsed_order_response.order_ids),
                ))
        }
        None => Ok(Response::new().add_attribute("method", "handle_place_order_reply")),
    }
}
