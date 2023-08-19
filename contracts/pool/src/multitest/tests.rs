#[cfg(test)]
mod test {

    use cosmwasm_std::{coin, coins, from_binary, Decimal};
    use sei_cosmwasm::{GetOrdersResponse, OrderType, PositionDirection};
    use sei_integration_tests::helper::{get_balance, mock_app};

    use crate::{
        contract::SEI_DEX_CONTRACT_ADDR,
        msg::QueryMsg,
        multitest::{
            alice, bob, charlie, init_default_balances, place_orders, query_balances, PoolCodeId,
            ATOM_DENOM, SEI_DENOM, USDT_DENOM,
        },
        SeiOrder,
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
        let price_denom = SEI_DENOM;
        let asset_denom = USDT_DENOM;
        let tick_size = 1000;
        let taker_fee_rate = 1;
        let maker_rebate_fee = 1;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                charlie(),
                price_denom,
                asset_denom,
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

        let price = 100u128;
        let quantity = 100u128;
        let leverage = 1u128;
        let position_effect = "Open";
        let status_description = "status_description";
        let nominal = 1u128;
        let funds = coins(10000, SEI_DENOM);

        let price_denom = SEI_DENOM;
        let asset_denom = ATOM_DENOM;
        let order_type = OrderType::Limit;
        let data = "".to_string();
        let position_direction = PositionDirection::Long;
        let status_description = "order1";
        let dex_contract_addr = SEI_DEX_CONTRACT_ADDR;

        // let resp = contract.limit_bid(
        //     &mut app,
        //     &alice(),
        //     price,
        //     quantity,
        //     leverage,
        //     position_effect,
        //     status_description,
        //     nominal,
        //     funds.as_slice(),
        // );

        // println!("resp: {:?}", resp);

        let orders = vec![SeiOrder {
            price: Decimal::raw(price),
            quantity: Decimal::raw(quantity),
            price_denom: price_denom.to_owned(),
            asset_denom: asset_denom.to_owned(),
            order_type,
            position_direction,
            data, // serialized order data, defined by the specific target contract
            status_description: status_description.to_string(),
            nominal: Decimal::raw(nominal),
        }];

        let resps = place_orders(&mut app, &charlie(), orders, funds, dex_contract_addr).unwrap();
        let res = resps.first().unwrap().clone().data;
        let data = res.unwrap();
        let out: String = from_binary(&data).unwrap();
        assert_eq!(out.to_string(), dex_contract_addr.to_string());

        let bids = contract.query_limit_bids(&app).unwrap().bids;
        assert_eq!(bids.len(), 1);

        let resp: GetOrdersResponse = app
            .wrap()
            .query_wasm_smart(
                contract.addr(),
                &QueryMsg::GetOrders {
                    contract_addr: SEI_DEX_CONTRACT_ADDR.to_string(),
                    account: contract.addr().to_string(),
                },
            )
            .unwrap();
        let orders = resp.orders;
        assert_eq!(orders.len(), 1);
    }
}
