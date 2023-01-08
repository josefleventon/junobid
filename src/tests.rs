#[cfg(test)]
mod tests {
    use crate::helpers::JunoBidContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const BIDDER: &str = "juno1bidder";
    const OTHER_BIDDER: &str = "juno1otherbidder";
    const ADMIN: &str = "juno1admin";
    const NATIVE_DENOM: &str = "ujunox";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(BIDDER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1500),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, JunoBidContract) {
        let mut app = mock_app();
        let juno_bid_id = app.store_code(contract());

        let msg = InstantiateMsg {
            admins: vec![Addr::unchecked(ADMIN)],
        };

        let juno_bid_contract_addr = app
            .instantiate_contract(juno_bid_id, Addr::unchecked(ADMIN), &msg, &[], "test", None)
            .unwrap();

        let juno_bid_contract = JunoBidContract(juno_bid_contract_addr);

        (app, juno_bid_contract)
    }

    mod tests {
        use cosmwasm_std::{coin, testing::mock_env, to_binary, CosmosMsg, Timestamp, WasmMsg};

        use super::*;
        use crate::{msg::ExecuteMsg, state::BiddingPeriod};

        #[test]
        fn try_start_bidding() {
            let (mut app, juno_bid_contract) = proper_instantiate();
            let env = mock_env();

            // Create a bidding period
            let msg = ExecuteMsg::StartBidding {
                config: BiddingPeriod {
                    name: "My Bidding Period".into(),
                    description: None,
                    expires_at: Timestamp::from_seconds(env.block.time.seconds() + 1440), // Expires in 24 hours,
                    minimum_bid: Uint128::new(500),
                    accepted_bidders: 1,
                    denom: NATIVE_DENOM.into(),
                },
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            }
            .into();

            app.execute(Addr::unchecked(ADMIN), juno_msg).unwrap();
        }

        #[test]
        fn try_end_bidding() {
            let (mut app, juno_bid_contract) = proper_instantiate();
            let env = mock_env();

            // Give OTHER_BIDDER 500 ujunox
            app.send_tokens(
                Addr::unchecked(BIDDER),
                Addr::unchecked(OTHER_BIDDER),
                &vec![coin(500, NATIVE_DENOM)],
            )
            .unwrap();

            // Create a bidding period
            let msg = ExecuteMsg::StartBidding {
                config: BiddingPeriod {
                    name: "My Bidding Period".into(),
                    description: None,
                    expires_at: Timestamp::from_seconds(env.block.time.seconds() + 1440), // Expires in 24 hours,
                    minimum_bid: Uint128::new(500),
                    accepted_bidders: 1,
                    denom: NATIVE_DENOM.into(),
                },
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            }
            .into();

            app.execute(Addr::unchecked(ADMIN), juno_msg).unwrap();

            // Create a bid for BIDDER
            let msg = ExecuteMsg::Bid { address: None };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(1000),
                }],
            }
            .into();

            app.execute(Addr::unchecked(BIDDER), juno_msg).unwrap();

            // Create a bid for OTHER_BIDDER
            let msg = ExecuteMsg::Bid { address: None };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(500),
                }],
            }
            .into();

            app.execute(Addr::unchecked(OTHER_BIDDER), juno_msg)
                .unwrap();

            // Accept the bid from BIDDER and end the bidding period
            let msg = ExecuteMsg::EndBidding {
                accepted_bids: vec![Addr::unchecked(BIDDER)],
                withdrawal_address: None,
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            }
            .into();

            app.execute(Addr::unchecked(ADMIN), juno_msg).unwrap();

            // Get the admin's balance
            // and verify that they have withdrawn 1000 ujunox
            let balance = app
                .wrap()
                .query_balance(Addr::unchecked(ADMIN), NATIVE_DENOM)
                .unwrap()
                .amount;

            assert_eq!(balance, Uint128::new(1000));

            // Get OTHER_BIDDER's balance
            // and verify that they have been refunded 500 ujunox
            let balance = app
                .wrap()
                .query_balance(Addr::unchecked(OTHER_BIDDER), NATIVE_DENOM)
                .unwrap()
                .amount;

            assert_eq!(balance, Uint128::new(500));
        }

        #[test]
        fn try_bid() {
            let (mut app, juno_bid_contract) = proper_instantiate();
            let env = mock_env();

            // Create a bidding period
            let msg = ExecuteMsg::StartBidding {
                config: BiddingPeriod {
                    name: "My Bidding Period".into(),
                    description: None,
                    expires_at: Timestamp::from_seconds(env.block.time.seconds() + 1440), // Expires in 24 hours,
                    minimum_bid: Uint128::new(500),
                    accepted_bidders: 1,
                    denom: NATIVE_DENOM.into(),
                },
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            }
            .into();

            app.execute(Addr::unchecked(ADMIN), juno_msg).unwrap();

            // Create a bid
            let msg = ExecuteMsg::Bid { address: None };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(1000),
                }],
            }
            .into();

            app.execute(Addr::unchecked(BIDDER), juno_msg).unwrap();
        }

        #[test]
        fn try_proxy_bid() {
            let (mut app, juno_bid_contract) = proper_instantiate();
            let env = mock_env();

            // Create a bidding period
            let msg = ExecuteMsg::StartBidding {
                config: BiddingPeriod {
                    name: "My Bidding Period".into(),
                    description: None,
                    expires_at: Timestamp::from_seconds(env.block.time.seconds() + 1440), // Expires in 24 hours,
                    minimum_bid: Uint128::new(500),
                    accepted_bidders: 1,
                    denom: NATIVE_DENOM.into(),
                },
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            }
            .into();

            app.execute(Addr::unchecked(ADMIN), juno_msg).unwrap();

            // Create a proxy bid
            let msg = ExecuteMsg::Bid {
                address: Some(Addr::unchecked(BIDDER)),
            };
            let juno_msg: CosmosMsg = WasmMsg::Execute {
                contract_addr: juno_bid_contract.addr().into(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(1000),
                }],
            }
            .into();

            app.execute(Addr::unchecked(BIDDER), juno_msg).unwrap();
        }
    }
}
