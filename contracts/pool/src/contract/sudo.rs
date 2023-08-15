use cosmwasm_std::{Binary, DepsMut, Env, Response, StdError};
use sei_cosmwasm::{
    BulkOrderPlacementsResponse, ContractOrderResult, DepositInfo, LiquidationRequest,
    LiquidationResponse, SeiMsg, SeiQueryWrapper, SettlementEntry,
};

use crate::{Order, SudoMsg};

pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<SeiMsg>, StdError> {
    match msg {
        SudoMsg::Settlement { epoch, entries } => process_settlements(deps, entries, epoch),
        SudoMsg::NewBlock { epoch } => handle_new_block(deps, env, epoch),
        SudoMsg::BulkOrderPlacements { orders, deposits } => {
            process_bulk_order_placements(deps, orders, deposits)
        }
        SudoMsg::BulkOrderCancellations { ids } => process_bulk_order_cancellations(deps, ids),
        SudoMsg::Liquidation { requests } => process_bulk_liquidation(deps, env, requests),
        SudoMsg::FinalizeBlock {
            contract_order_results,
        } => process_finalize_block(deps, env, contract_order_results),
    }
}

pub fn process_settlements(
    _deps: DepsMut<SeiQueryWrapper>,
    _entries: Vec<SettlementEntry>,
    _epoch: i64,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn handle_new_block(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _epoch: i64,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_order_placements(
    deps: DepsMut<SeiQueryWrapper>,
    _orders: Vec<Order>,
    _deposits: Vec<DepositInfo>,
) -> Result<Response<SeiMsg>, StdError> {
    let response = BulkOrderPlacementsResponse {
        unsuccessful_orders: vec![],
    };
    let serialized_json = match serde_json::to_string(&response) {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing response: {:?}", error),
    };
    let base64_json_str = base64::encode(serialized_json);
    let binary = match Binary::from_base64(base64_json_str.as_ref()) {
        Ok(val) => val,
        Err(error) => panic!("Problem converting binary for order request: {:?}", error),
    };

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    deps.api
        .debug(&format!("process_bulk_order_placements: {:?}", response));
    return Ok(Response::new());
}

pub fn process_bulk_order_cancellations(
    _deps: DepsMut<SeiQueryWrapper>,
    _ids: Vec<u64>,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_liquidation(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _requests: Vec<LiquidationRequest>,
) -> Result<Response<SeiMsg>, StdError> {
    let response = LiquidationResponse {
        successful_accounts: vec![],
        liquidation_orders: vec![],
    };
    let serialized_json = match serde_json::to_string(&response) {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing response: {:?}", error),
    };
    let base64_json_str = base64::encode(serialized_json);
    let binary = match Binary::from_base64(base64_json_str.as_ref()) {
        Ok(val) => val,
        Err(error) => panic!("Problem converting binary for order request: {:?}", error),
    };

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    deps.api.debug(&format!(
        "pub fn process_bulk_liquidation(
            : {:?}",
        response
    ));
    return Ok(Response::new());
}

pub fn process_finalize_block(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    contract_order_results: Vec<ContractOrderResult>,
) -> Result<Response<SeiMsg>, StdError> {
    deps.api.debug("Processing finalize block...");

    // print order placement results
    for order_results in contract_order_results {
        deps.api.debug(&format!(
            "Order results from contract {}",
            order_results.contract_address
        ));

        for order_placement in order_results.order_placement_results {
            deps.api.debug(&format!(
                "Order id {}, status {}",
                order_placement.order_id, order_placement.status_code
            ));
        }
        for order_execution in order_results.order_execution_results {
            deps.api.debug(&format!(
                "Order id {}, executed_quantity {}",
                order_execution.order_id, order_execution.executed_quantity
            ));
        }
    }

    let response = Response::new();
    Ok(response)
}
