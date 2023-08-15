use cosmwasm_std::{ensure, Addr, Coin, MessageInfo};

use crate::{state::State, ContractError};

pub type UnitResult = Result<(), ContractError>;

pub fn validate_base_denom(state: &State, denom: &str) -> UnitResult {
    ensure!(
        denom == state.base_denom,
        ContractError::UnSupportedDenom {
            denom: denom.into(),
        }
    );
    Ok(())
}

pub fn validate_quote_denom(state: &State, denom: &str) -> UnitResult {
    ensure!(
        denom == state.quote_denom,
        ContractError::UnSupportedDenom {
            denom: denom.into(),
        }
    );
    Ok(())
}

pub fn validate_owner(owner: &Addr, info: &MessageInfo) -> UnitResult {
    ensure!(owner == info.sender, ContractError::Unauthorized {});

    Ok(())
}

pub fn validate_balance(balance: &Coin, to_withdraw: u128) -> UnitResult {
    ensure!(
        balance.amount.u128() >= to_withdraw,
        ContractError::BalanceTooSmall {
            balance: balance.to_owned()
        }
    );
    Ok(())
}

pub fn validate_position_effect(position_effect: &str) -> UnitResult {
    let position_effect = position_effect.trim().to_lowercase();
    let position_effect = position_effect.as_str();
    ensure!(
        position_effect == "open" || position_effect == "close" || position_effect == "unknown",
        ContractError::InvalidPositionEffect {
            position_effect: position_effect.into(),
        }
    );
    Ok(())
}
