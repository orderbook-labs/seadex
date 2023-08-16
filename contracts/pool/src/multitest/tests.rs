#[cfg(test)]
mod test {

    use cosmwasm_std::coin;
    use sei_integration_tests::helper::{get_balance, mock_app};

    use crate::multitest::{
        alice, bob, charlie, init_default_balances, query_balances, PoolCodeId, ATOM_DENOM,
        SEI_DENOM, USDT_DENOM,
    };

    #[test]
    fn instantiate_should_works() {
        // Initialize
        let mut app = mock_app(init_default_balances, vec![]);

        let code_id = PoolCodeId::store_code(&mut app);
        let base_denom = SEI_DENOM;
        let quote_denom = USDT_DENOM;
        let tick_size = 1000;
        let taker_fee_rate = 1;
        let maker_rebate_fee = 1;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                charlie(),
                base_denom,
                quote_denom,
                tick_size,
                taker_fee_rate,
                maker_rebate_fee,
                label,
            )
            .unwrap();

        let balances = query_balances(&app, contract.addr()).unwrap();
        assert!(balances.is_empty());

        let alice_sei_balance = get_balance(&app, alice().to_string(), SEI_DENOM.to_owned());
        assert_eq!(alice_sei_balance.amount, coin(10_000_000, SEI_DENOM));
        let alice_atom_balance = get_balance(&app, alice().to_string(), ATOM_DENOM.to_owned());
        assert_eq!(alice_atom_balance.amount, coin(10_000_000, ATOM_DENOM));

        let bob_sei_balance = get_balance(&app, bob().to_string(), SEI_DENOM.to_owned());
        assert_eq!(bob_sei_balance.amount, coin(10_000_000, SEI_DENOM));
        let bob_atom_balance = get_balance(&app, bob().to_string(), ATOM_DENOM.to_owned());
        assert_eq!(bob_atom_balance.amount, coin(10_000_000, ATOM_DENOM));

        let owner = contract.owner(&app).unwrap();
        assert_eq!(owner.owner, charlie());

        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.next_bid_id, 0);
        assert_eq!(state.next_ask_id, 0);
        assert_eq!(state.tick_size, 1000);
    }

    #[test]
    fn swap_pool_full_flows_should_works() {
        let mut app = mock_app(init_default_balances, vec![]);

        let code_id = PoolCodeId::store_code(&mut app);
        let base_denom = SEI_DENOM;
        let quote_denom = USDT_DENOM;
        let tick_size = 1000;
        let taker_fee_rate = 1;
        let maker_rebate_fee = 1;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                charlie(),
                base_denom,
                quote_denom,
                tick_size,
                taker_fee_rate,
                maker_rebate_fee,
                label,
            )
            .unwrap();

        let balances = query_balances(&app, contract.addr()).unwrap();
        assert!(balances.is_empty());

        let alice_sei_balance = get_balance(&app, alice().to_string(), SEI_DENOM.to_owned());
        assert_eq!(alice_sei_balance.amount, coin(10_000_000, SEI_DENOM));
        let alice_atom_balance = get_balance(&app, alice().to_string(), ATOM_DENOM.to_owned());
        assert_eq!(alice_atom_balance.amount, coin(10_000_000, ATOM_DENOM));

        let bob_sei_balance = get_balance(&app, bob().to_string(), SEI_DENOM.to_owned());
        assert_eq!(bob_sei_balance.amount, coin(10_000_000, SEI_DENOM));
        let bob_atom_balance = get_balance(&app, bob().to_string(), ATOM_DENOM.to_owned());
        assert_eq!(bob_atom_balance.amount, coin(10_000_000, ATOM_DENOM));
    }
}
