use std::str::FromStr;

use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;

// a contract specific order data struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub leverage: Decimal,
    pub position_effect: PositionEffect,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema, Eq, Hash)]
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
            _ => Err(ContractError::InvalidPositionEffect {
                position_effect: s.into(),
            }),
        }
    }
}
