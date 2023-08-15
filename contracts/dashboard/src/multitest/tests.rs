#[cfg(test)]
mod test {
    use cw_multi_test::App;
    use pool::multitest::{PoolCodeId, SEI_DENOM, USDT_DENOM};

    // use crate::multitest::{alice, bob, owner, DashboardCodeId, DashboardContract};

    #[test]
    fn dashboard_instantiate_should_works() {
        let mut app = App::default();
        let code_id = DashboardCodeId::store_code(&mut app);
        let pool_code_id = PoolCodeId::store_code(&mut app);
        let name = "Dashoard";
        let label = "Pool label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, pool_code_id.into(), label)
            .unwrap();

        // check owner
        let contract_owner = contract.owner(&app).unwrap();
        assert_eq!(contract_owner.owner, owner());

        // check state
        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.name, "Dashboard");
        assert_eq!(state.pools_count, 0);

        // check balances
        let balances = DashboardContract::query_balances(&app, contract.addr()).unwrap();
        assert!(balances.is_empty());
    }

    #[test]
    fn dashboard_create_pool_should_works() {
        let mut app = App::default();
        let code_id = DashboardCodeId::store_code(&mut app);
        let lottery_code_id = PoolCodeId::store_code(&mut app);
        let name = "Dashoard";
        let label = "Pool label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, lottery_code_id.into(), label)
            .unwrap();

        let base_denom = SEI_DENOM;
        let quote_denom = USDT_DENOM;
        let tick_size = 1000;
        let taker_fee_rate = 1;
        let maker_rebate_fee = 1;
        let label = format!("{base_denom}-{quote_denom}-pool, {:?}", label);

        let resp = contract
            .create_pool(
                &mut app,
                owner(),
                base_denom,
                quote_denom,
                tick_size,
                taker_fee_rate,
                maker_rebate_fee,
                &label,
            )
            .unwrap();

        println!("create pool resp:{:?}", resp);

        let pool_addr = resp.unwrap().addr;

        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.pools_count, 1);

        let pools = contract.pools(&app).unwrap();
        assert_eq!(pools.lotteries.len(), 1);

        let pool = &pools.lotteries[0];
        assert_eq!(pool.base_denom, base_denom);
        assert_eq!(pool.quote_denom, quote_denom);
        assert_eq!(pool.tick_size, tick_size);
        assert_eq!(pool.taker_fee_rate, taker_fee_rate);
        assert_eq!(pool.maker_rebate_fee, maker_rebate_fee);
        assert_eq!(pool.created_by, owner());
        assert_eq!(pool.contract_addr, pool_addr);
    }
}
