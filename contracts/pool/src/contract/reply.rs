use cosmwasm_std::{from_binary, DepsMut, Env, Reply, Response, StdError, SubMsgResponse};
use cw_utils::{parse_execute_response_data, parse_instantiate_response_data};
use protobuf::Message;
use sei_cosmwasm::{MsgPlaceOrdersResponse, SeiMsg, SeiQueryWrapper};

use crate::ContractError;

use super::PLACE_ORDER_REPLY_ID;

pub fn reply(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    reply: Reply,
) -> Result<Response<SeiMsg>, ContractError> {
    match reply.id {
        PLACE_ORDER_REPLY_ID => Ok(handle_place_order_reply(deps, reply.result.into_result())?),
        id => Err(ContractError::UnRecognizedReplyId { id }),
    }
}

pub fn handle_place_order_reply(
    deps: DepsMut<SeiQueryWrapper>,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response<SeiMsg>, StdError> {
    println!("reply is: {:?}", reply);

    // let reply = reply.map_err(StdError::generic_err)?.data;

    // let resp = reply.map(|data| parse_execute_response_data(&data)).transpose().unwrap();

    // let resp = resp.map(|data| from_binary(&data.data.unwrap())).transpose()?;

    // let resp: Option<Addr> = reply.map_err(StdError::generic_err)?
    //     .data
    //         .map(|data| parse_execute_response_data(&data))
    //         .transpose()?
    //         .and_then(|data| data.data)
    //         .map(|data| from_binary(&data))
    //         .transpose()
    //         .map_err(Into::into)?;

    // Ok(Response::new())
    // todo!()

    let submsg_response: SubMsgResponse = reply.map_err(StdError::generic_err)?;

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
