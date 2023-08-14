#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{from_binary, Addr, Coin, StdResult};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query, reply},
    msg::*,
};

#[derive(Clone, Debug, Copy)]
pub struct DashboardCodeId(u64);

impl DashboardCodeId {
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
        name: &str,
        lottery_code_id: u64,
        label: &str,
    ) -> AnyResult<DashboardContract> {
        DashboardContract::instantiate(app, self, sender, name, lottery_code_id, label)
    }
}

impl From<DashboardCodeId> for u64 {
    fn from(code_id: DashboardCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct DashboardContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl DashboardContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: DashboardCodeId,
        sender: Addr,
        name: &str,
        pool_code_id: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(name, pool_code_id);

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

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn create_pool(
        &self,
        app: &mut App,
        sender: Addr,
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        label: &str,
    ) -> AnyResult<Option<InstantiationData>> {
        let msg = ExecuteMsg::CreatePool {
            base_denom: base_denom.into(),
            quote_denom: quote_denom.into(),
            tick_size,
            taker_fee_rate,
            maker_rebate_rate: maker_rebate_fee,
            label: label.into(),
        };

        let resp = app
            .execute_contract(sender, self.addr(), &msg, &[])
            .unwrap();

        // println!("execute create lottery resp:{:?}", resp);

        let data = from_binary(&resp.data.unwrap()).unwrap();

        Ok(data)
    }

    #[track_caller]
    pub fn freeze_pool(&self, app: &mut App, sender: Addr, pool: &str) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender,
            self.addr(),
            &ExecuteMsg::FreezePool {
                lottery: pool.into(),
            },
            &[],
        )
    }

    pub fn pools(&self, app: &App) -> StdResult<PoolsResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Pools {})
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

impl From<Addr> for DashboardContract {
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
