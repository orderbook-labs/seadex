#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query, reply},
    msg::*,
};

pub const SEI_DENOM: &str = "usei";
pub const SEI_DECIMALS: u8 = 18;

pub const USDC_DENOM: &str = "usdc";
pub const USDT_DENOM: &str = "usdt";
pub const BTC_DENOM: &str = "btc";
pub const ETH_DENOM: &str = "eth";

#[derive(Clone, Debug, Copy)]
pub struct PoolCodeId(u64);

impl PoolCodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        let code_id = app.store_code(Box::new(contract));
        Self(code_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: Addr,
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        label: &str,
    ) -> AnyResult<PoolContract> {
        PoolContract::instantiate(
            app,
            self,
            sender,
            base_denom,
            quote_denom,
            tick_size,
            taker_fee_rate,
            maker_rebate_fee,
            label,
        )
    }
}

impl From<PoolCodeId> for u64 {
    fn from(code_id: PoolCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct PoolContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl PoolContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: PoolCodeId,
        sender: Addr,
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_rate: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(
            base_denom,
            quote_denom,
            tick_size,
            taker_fee_rate,
            maker_rebate_rate,
        );

        app.instantiate_contract(
            code_id.0,
            Addr::unchecked(sender),
            &init_msg,
            &[],
            label,
            None,
        )
        .map(Self::from)
    }

    #[track_caller]
    pub fn make_market() -> AnyResult<AppResponse> {
        todo!()
    }

    pub fn owner(&self, app: &App) -> StdResult<OwnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Owner {})
    }

    pub fn query_balances(app: &App, addr: Addr) -> StdResult<Vec<Coin>> {
        app.wrap().query_all_balances(addr)
    }

    pub fn query_state(&self, app: &App) -> StdResult<CurrentStateResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::CurrentState {})
    }
}

impl From<Addr> for PoolContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}

pub fn alice() -> Addr {
    Addr::unchecked("sei1vqm0e2t4yefty2haha4ryr42zpyxqk8257kag5")
}

pub fn bob() -> Addr {
    Addr::unchecked("sei1aan9kqywf4rf274cal0hj6eyly6wu0uv7edxy2")
}

pub fn owner() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn parent() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}
