#[cfg(test)]
mod tests {
    use crate::{
        admin::execute::AdminExecuteMsg,
        incentives::{execute::IncentivesExecuteMsg, CoinVec},
        msg::ExecuteMsg,
        state::{ClaimAccount, MerkleProof},
        test_tube::initialize::initialize::default_init,
    };
    use cosmwasm_std::Coin;
    use merkle::{hash::Hash, proof::Entry};
    use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
    use osmosis_test_tube::{
        osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
        Account, Bank, Module, Wasm,
    };

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init();
        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // Create accounts
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/fetch/100001.json#L3
        let claim_accounts: Vec<ClaimAccount> = vec![
            // osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt
            ClaimAccount {
                mnemonic: "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("e18PIQzBUqdhbHzoLCuZpJ/tLCXVk3HdKZPJ+u3+3i8=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("4cjXFSw+17blv/GUE33Tdoc362PUY27qSqvuGGHqGNA=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("7rzV1PgDxiF8N2hRR52P2Rse6Yp4Aht2L66O4kNk2sM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "uosmo")])
            },
            // osmo1w8rqvadvarr0hqlpah6t4efc8mam44q8yd3qnt
            ClaimAccount {
                mnemonic: "include monster floor galaxy skate second sister offer silver another upset mind frame into suit velvet lonely butter cousin side bridge answer logic pole",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("qQEesKAbniv8rw0b7HVsVDF7eQP25Q/8WBmeB1Vh2z0=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("4cjXFSw+17blv/GUE33Tdoc362PUY27qSqvuGGHqGNA=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("7rzV1PgDxiF8N2hRR52P2Rse6Yp4Aht2L66O4kNk2sM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "uosmo")])
            },
            // osmo1gg7s5w4cnpqrd6g8njrvh8vhh6fy2wt7mau0s9
            ClaimAccount {
                mnemonic: "tide indoor kid review skin over try drive flower off inquiry winter summer what stick high memory atom hard deer kitchen must concert dizzy",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("WJVJzzdRE80W4NwQaUb+Os2hnfFSrBg0QC0HSdxT1IM=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("KF/OwwEO9DPRZ6BD2HgZgPxhBQU+ZaeE0Ln/VIUaXSI=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("7rzV1PgDxiF8N2hRR52P2Rse6Yp4Aht2L66O4kNk2sM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "uosmo")])
            },
            // osmo1mxn37ce4n6uy9hvp974ncpdgrqvevma4lm90mx
            ClaimAccount {
                mnemonic: "wrong join rifle knee myth woman layer actor question hockey fitness ignore bleak announce arrow crazy dish orbit divide melody pattern kiwi brown lawn",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("dekOXSunmM9nlCn4DMv/kkSE7HnsObqQTfNNsi8YLcs=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("KF/OwwEO9DPRZ6BD2HgZgPxhBQU+ZaeE0Ln/VIUaXSI=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("7rzV1PgDxiF8N2hRR52P2Rse6Yp4Aht2L66O4kNk2sM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "uosmo")])
            },
            // osmo1jsw73ff43vnv8qr8023y7xc5mnpq5juuw9qwak
            ClaimAccount {
                mnemonic: "boy casual file warfare family report embrace piece jewel garment loop device collect insane year flock swift open lobster infant antenna asset alcohol solid",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("/f92UCk/4O7wQHT3MCwP5L4W1ySYAP9PuV7C5uKlaLU=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("DrezS3jfIXK93eEoqDPNlCecFtJUsN6KyFcA7ZOB1W0=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("J1k5ruKtbyR25XtJtB1qSI9iUuflbEecowe2/ozhBOE=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000000, "uosmo")])
            },
            // osmo19g859hauf65wyfuj5z439q5hulyzjck8yqze24
            ClaimAccount {
                mnemonic: "tail praise mansion there pause cube poverty chalk dizzy dinner reveal electric mistake clever present pink blade cram coyote banana dog cargo cook someone",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("ZdBECXQyherBUIZP9sqB3cPrqmIRSxeexOGkZNXQejw=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("DrezS3jfIXK93eEoqDPNlCecFtJUsN6KyFcA7ZOB1W0=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("J1k5ruKtbyR25XtJtB1qSI9iUuflbEecowe2/ozhBOE=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000000, "uosmo")])
            },
            // osmo13p2vtgvklqg0a75snzkurxk79lqtlmdjr7uta8
            ClaimAccount {
                mnemonic: "brown alley chunk iron stem they piece conduct near dirt poet truth clinic shallow pen above merit trophy gauge clerk excite evoke hour allow",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("fKAeSWYdJ3bYR2cCLgZrHKoGaxyoT6NQ1EDc0PwfPn0=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("HEBddmUBzdT/umNBPGiFCuS+PDUXu3WtOWIW4FXGAg8=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("J1k5ruKtbyR25XtJtB1qSI9iUuflbEecowe2/ozhBOE=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000000, "uosmo")])
            },
            // osmo1fa3y3m2njlhlr6z9mhqxg062n0n2e8qsl7kdwa
            ClaimAccount {
                mnemonic: "humor exclude bulk trim fade sun moral mention topple keen nation convince desk tongue fish hill craft increase snack glass rural gate cheap mention",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XCO4OtZSF+yHVS+JAi1JqhTbCUZjP1rlR0lpHOo/DVg=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("HEBddmUBzdT/umNBPGiFCuS+PDUXu3WtOWIW4FXGAg8=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("J1k5ruKtbyR25XtJtB1qSI9iUuflbEecowe2/ozhBOE=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MwU7KsQJFyWWjYMDw/y829pyD2vbkDjWFgyeXBTX/9Y=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000900000, "uosmo")])
            },
            // osmo1hkd3272hf7jw90x980s3t8hzcdxar0r3e5ahfs
            ClaimAccount {
                mnemonic: "perfect food future blush oak shrug tank under state illegal object awake erode poet tuition athlete answer sheriff say knee later fat dress visa",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("0EuIeYhwQWnkzsruGNkw31434GXDOkjApNeGmuqPrX0=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("B1Zaw00R7+LYspNmwQLeX185pK66n7UQB+mgJczwsoM=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000009000000, "uosmo")])
            },
            // osmo1uxxt4z229zdpq96ljadd2vzzxkp960t0p3lh9k
            ClaimAccount {
                mnemonic: "smoke jealous man occur grief hat tobacco hospital fruit raise path primary secret budget wait police black panel resemble ten garden coach artefact engage",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("S2hynVcAP9BQUOTFdthBZI125t/TxNSrg+9rMJ+R/h8=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("B1Zaw00R7+LYspNmwQLeX185pK66n7UQB+mgJczwsoM=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000090000000, "uosmo")])
            }
        ];

        // Fund the incentives contract
        let _ = bank.send(
            MsgSend {
                from_address: admin.address().to_string(),
                to_address: contract.to_string(),
                amount: vec![OsmoCoin {
                    amount: "1000000000000000000".to_string(), // 1_000_000_000_000.000000 OSMO (1T $OSMO)
                    denom: "uosmo".to_string(),
                }],
            },
            &admin,
        );

        // Assert initial balance on gauge contract
        let contract_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: contract.to_string(),
                denom: "uosmo".to_string(),
            })
            .unwrap();
        assert_eq!(
            contract_balance.balance.unwrap().amount,
            "1000000000000000000".to_string()
        );

        // Execute AdminMsg::UpdateAdmin
        let new_admin = app
            .init_account(&[Coin::new(1_000_000_000, "uosmo")])
            .unwrap();
        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateAdmin {
                    new_admin: new_admin.address(),
                }),
                &[],
                &admin,
            )
            .unwrap();

        // TODO: Assert admin changed and queriable

        // AdminMsg::UpdateMerkleRoot
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/merkle/100001.json#L3
        let merkle_root: &str = "yZsdqUjU3p9Iy7Df1Eon5Op+2d08cHVokMX5bvA3I00=";
        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateMerkleRoot {
                    new_root: merkle_root.to_string(),
                }),
                &[],
                &new_admin,
            )
            .unwrap();

        // TODO: Assert merkle root changed and queriable
        // let response = wasm
        //     .query(
        //         contract.as_str(),
        //         &QueryMsg::IncentivesQuery(IncentivesQueryMsg::GetMerkleRoot {})
        //     )
        //     .unwrap();

        // Execute IncentivesMsg::Claim
        for claim_account in claim_accounts {
            let account = app
                .init_account_from_mnemonic(
                    &[Coin::new(100_000_000_000_000_000_000, "uosmo")],
                    claim_account.mnemonic,
                )
                .unwrap();

            println!("claim_account.proofs {:?}", claim_account.proofs);
            let mut entries: Vec<Entry> = vec![];
            for proof in claim_account.proofs {
                entries.push(Entry {
                    is_left_sibling: proof.is_left_sibling,
                    hash: Hash::from(proof.hash),
                })
            }
            let proof_string = serde_json_wasm::to_string(&entries).unwrap();
            println!("proof_string {:?}", proof_string);

            // Execute claim for the current user
            let _ = wasm
                .execute(
                    contract.as_str(),
                    &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                        address: account.address(),
                        coins: claim_account.coins.clone(),
                        proof: proof_string.to_string(),
                    }),
                    &[],
                    &account,
                )
                .unwrap();

            // Assert bank send occurred
            let address_balance = bank
                .query_balance(&QueryBalanceRequest {
                    address: account.address().to_string(),
                    denom: "uosmo".to_string(),
                })
                .unwrap();
            assert_eq!(
                address_balance.balance.unwrap().amount,
                claim_account
                    .coins
                    .coins()
                    .get(0)
                    .unwrap()
                    .amount
                    .to_string()
            );
        }

        // Assert final balance on gauge contract
        // Positive due to precision loss
        let contract_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: contract.to_string(),
                denom: "uosmo".to_string(),
            })
            .unwrap();
        assert_eq!(
            contract_balance.balance.unwrap().amount,
            "100000".to_string()
        );
    }
}
