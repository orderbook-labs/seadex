use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Timestamp};

use crate::{ContractError, SeiOrder};

#[cw_serde]
pub struct Order {
    pub id: u64,
    pub order: SeiOrder,
    pub create_time: Timestamp,
    pub owner: Addr,
}

// a contract specific order data struct
#[cw_serde]
pub struct OrderData {
    pub leverage: Decimal,
    pub position_effect: PositionEffect,
}

#[cw_serde]
#[derive(Copy, Eq, Hash)]
pub enum PositionEffect {
    Unknown,
    Open,
    Close,
}

impl FromStr for PositionEffect {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "open" => Ok(PositionEffect::Open),
            "close" => Ok(PositionEffect::Close),
            "unknown" => Ok(PositionEffect::Unknown),
            _ => Err(ContractError::InvalidPositionEffect { position_effect: s }),
        }
    }
}
