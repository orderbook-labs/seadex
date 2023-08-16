#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Addr, Api, Coin, Empty, GovMsg, IbcMsg, IbcQuery, StdResult, Storage,
};

use cw_multi_test::{
    App, AppResponse, BankKeeper, ContractWrapper, DistributionKeeper, Executor, FailingModule,
    Router, StakeKeeper, WasmKeeper,
};

use sei_cosmwasm::{SeiMsg, SeiQueryWrapper};

use sei_integration_tests::module::SeiModule;

use crate::{
    contract::{execute, instantiate, query, reply, sudo},
    msg::*,
};

pub const SEI_DENOM: &str = "usei";
pub const SEI_DECIMALS: u8 = 18;

pub const USDC_DENOM: &str = "usdc";
pub const USDT_DENOM: &str = "usdt";
pub const BTC_DENOM: &str = "btc";
pub const ETH_DENOM: &str = "eth";
pub const ATOM_DENOM: &str = "uatom";

pub type SeiRouter = Router<
    BankKeeper,
    SeiModule,
    WasmKeeper<SeiMsg, SeiQueryWrapper>,
    StakeKeeper,
    DistributionKeeper,
    FailingModule<IbcMsg, IbcQuery, Empty>,
    FailingModule<GovMsg, Empty, Empty>,
>;

pub type SeiApp = App<
    BankKeeper,
    MockApi,
    MockStorage,
    SeiModule,
    WasmKeeper<SeiMsg, SeiQueryWrapper>,
    StakeKeeper,
    DistributionKeeper,
    FailingModule<IbcMsg, IbcQuery, Empty>,
    FailingModule<GovMsg, Empty, Empty>,
>;

#[derive(Clone, Debug, Copy)]
pub struct PoolCodeId(u64);

impl PoolCodeId {
    pub fn store_code(app: &mut SeiApp) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query)
            .with_reply(reply)
            .with_sudo(sudo);

        let code_id = app.store_code(Box::new(contract));
        Self(code_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut SeiApp,
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
        app: &mut SeiApp,
        code_id: PoolCodeId,
        sender: Addr,
        base_denom: impl Into<String>,
        quote_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(
            base_denom,
            quote_denom,
            tick_size,
            taker_fee_rate,
            maker_rebate_fee,
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

    pub fn owner(&self, app: &SeiApp) -> StdResult<OwnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Owner {})
    }

    pub fn query_state(&self, app: &SeiApp) -> StdResult<CurrentStateResp> {
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

pub fn charlie() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn admin() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}

pub fn init_default_balances(router: &mut SeiRouter, _api: &dyn Api, storage: &mut dyn Storage) {
    router
        .bank
        .init_balance(
            storage,
            &admin(),
            vec![coin(1_000_000_000_000_000, SEI_DENOM.to_string())],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &alice(),
            vec![
                coin(10_000_000, SEI_DENOM.to_string()),
                coin(10_000_000, ATOM_DENOM.to_string()),
            ],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &bob(),
            vec![
                coin(10_000_000, SEI_DENOM.to_string()),
                coin(10_000_000, ATOM_DENOM.to_string()),
            ],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &charlie(),
            vec![
                coin(10_000_000, SEI_DENOM.to_string()),
                coin(10_000_000, ATOM_DENOM.to_string()),
            ],
        )
        .unwrap();
}

pub fn init_contract(
    app: &mut SeiApp,
    code_id: u64,
    sender: Addr,
    init_msg: &InstantiateMsg,
    send_funds: &[Coin],
    label: &str,
    admin: Option<String>,
) -> Addr {
    app.instantiate_contract(code_id, sender, init_msg, send_funds, label, admin)
        .unwrap()
}

pub fn query_balances(app: &SeiApp, addr: Addr) -> StdResult<Vec<Coin>> {
    app.wrap().query_all_balances(addr)
}
