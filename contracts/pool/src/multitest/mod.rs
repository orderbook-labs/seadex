#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Addr, Api, Coin, CosmosMsg, Empty, GovMsg, IbcMsg, IbcQuery, StdResult, Storage,
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
    SeiOrder,
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
        price_denom: impl Into<String>,
        asset_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        label: &str,
    ) -> AnyResult<PoolContract> {
        PoolContract::instantiate(
            app,
            self,
            sender,
            price_denom,
            asset_denom,
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
        price_denom: impl Into<String>,
        asset_denom: impl Into<String>,
        tick_size: u64,
        taker_fee_rate: u64,
        maker_rebate_fee: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(
            price_denom,
            asset_denom,
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
    #[allow(clippy::too_many_arguments)]
    pub fn limit_bid(
        &self,
        app: &mut SeiApp,
        sender: &Addr,
        price: u128,
        quantity: u128,
        leverage: u128,
        position_effect: &str,
        status_description: &str,
        nominal: u128,
        funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.addr(),
            &ExecuteMsg::LimitBid {
                price,
                quantity,
                leverage,
                position_effect: position_effect.to_owned(),
                status_description: status_description.to_owned(),
                nominal,
            },
            funds,
        )
    }

    pub fn owner(&self, app: &SeiApp) -> StdResult<OwnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Owner {})
    }

    pub fn query_state(&self, app: &SeiApp) -> StdResult<CurrentStateResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::CurrentState {})
    }

    pub fn query_limit_bids(&self, app: &SeiApp) -> StdResult<LimitBidsResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::LimitBids {})
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

#[track_caller]
pub fn place_orders(
    app: &mut SeiApp,
    sender: &Addr,
    orders: Vec<SeiOrder>,
    funds: Vec<Coin>,
    dex_contract_addr: &str,
) -> AnyResult<Vec<AppResponse>> {
    app.execute_multi(
        sender.to_owned(),
        vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
            orders,
            funds,
            contract_address: Addr::unchecked(dex_contract_addr),
        })],
    )
}
