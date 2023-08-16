use cosmwasm_schema::cw_serde;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    LimitBid {
        price: u128,
        quantity: u128,
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
