// #[cfg(test)]
// mod test {

//     use cosmwasm_std::coins;
//     use cw_multi_test::App;

//     use crate::multitest::{
//         alice, bob, owner, parent, PoolCodeId, PoolContract, SEI_DENOM, USDT_DENOM,
//     };
//     // use cw721_base::multi_tests;

//     #[test]
//     fn instantiate_should_works() {
//         // Initialize
//         let mut app = App::new(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &alice(), coins(300, SEI_DENOM))
//                 .unwrap();
//             router
//                 .bank
//                 .init_balance(storage, &bob(), coins(500, SEI_DENOM))
//                 .unwrap();
//         });

//         let code_id = PoolCodeId::store_code(&mut app);
//         let base_denom = SEI_DENOM;
//         let quote_denom = USDT_DENOM;
//         let tick_size = 1000;
//         let taker_fee_rate = 1;
//         let maker_rebate_fee = 1;
//         let label = "Lottery label";
//         let contract = code_id
//             .instantiate(
//                 &mut app,
//                 owner(),
//                 base_denom,
//                 quote_denom,
//                 tick_size,
//                 taker_fee_rate,
//                 maker_rebate_fee,
//                 label,
//             )
//             .unwrap();

//         let balances = PoolContract::query_balances(&app, contract.addr()).unwrap();
//         assert!(balances.is_empty());

//         let alice_balances = PoolContract::query_balances(&app, alice()).unwrap();
//         assert_eq!(alice_balances, coins(200, SEI_DENOM));

//         let bob_balances = PoolContract::query_balances(&app, bob()).unwrap();
//         assert_eq!(bob_balances, coins(500, SEI_DENOM));

//         let parent_balances = PoolContract::query_balances(&app, parent()).unwrap();
//         assert_eq!(parent_balances, coins(100, SEI_DENOM));
//     }

//     #[test]
//     fn swap_pool_full_flows_should_works() {
//         // Initialize
//         let mut app = App::new(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &alice(), coins(300, SEI_DENOM))
//                 .unwrap();
//             router
//                 .bank
//                 .init_balance(storage, &bob(), coins(500, SEI_DENOM))
//                 .unwrap();
//         });

//         let code_id = PoolCodeId::store_code(&mut app);
//         let base_denom = SEI_DENOM;
//         let quote_denom = USDT_DENOM;
//         let tick_size = 1000;
//         let taker_fee_rate = 1;
//         let maker_rebate_fee = 1;
//         let label = "Lottery label";
//         let contract = code_id
//             .instantiate(
//                 &mut app,
//                 owner(),
//                 base_denom,
//                 quote_denom,
//                 tick_size,
//                 taker_fee_rate,
//                 maker_rebate_fee,
//                 label,
//             )
//             .unwrap();

//         let balances = PoolContract::query_balances(&app, contract.addr()).unwrap();
//         assert!(balances.is_empty());

//         let alice_balances = PoolContract::query_balances(&app, alice()).unwrap();
//         assert_eq!(alice_balances, coins(200, SEI_DENOM));

//         let bob_balances = PoolContract::query_balances(&app, bob()).unwrap();
//         assert_eq!(bob_balances, coins(500, SEI_DENOM));

//         let parent_balances = PoolContract::query_balances(&app, parent()).unwrap();
//         assert_eq!(parent_balances, coins(100, SEI_DENOM));
//     }
// }
