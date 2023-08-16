use cosmwasm_schema::cw_serde;

use crate::ContractError;

use sei_cosmwasm::SeiMsg;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    LimitBid {
        price: u128,
        quantity: u128,
        // price_denom: String,
        // asset_denom: String,
        leverage: u128,
        position_effect: String, // Open, Close or Unknown
        status_description: String,
        nominal: u128,
    },
    MarketBid {},
    LimitAsk {},
    MarketAsk {},
    MakeMarket {},
    CancelOrders {
        order_ids: Vec<u64>,
    },
    SetDexContract {
        addr: String,
    },
    // PlaceOrders {},
    // CancelOrders { order_ids: Vec<u64> },
    // CreateDenom {},
    // Mint {},
    // Burn {},
    // ChangeAdmin {},
    // SetMetadata {},
}

impl TryFrom<ExecuteMsg> for SeiMsg {
    type Error = ContractError;

    fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
        use ExecuteMsg::*;

        match msg {
            LimitAsk {} => Ok(SeiMsg::PlaceOrders {
                orders: todo!(),
                funds: todo!(),
                contract_address: todo!(),
            }),
            _ => todo!(),
        }
    }
}
