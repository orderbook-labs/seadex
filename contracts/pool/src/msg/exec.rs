use cosmwasm_schema::cw_serde;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    LimitBid {},
    MarketBid {},
    LimitAsk {},
    MarketAsk {},
    MakeMarket {},
    CancelOrder { order_id: u128 },
    CancelAllOrders {},
    Freeze {},
}
