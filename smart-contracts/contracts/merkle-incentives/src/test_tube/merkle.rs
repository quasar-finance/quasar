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
            // osmo16wxzza5p3wvy8rdnyfdq80zhe6we9xpmmgllkz
            ClaimAccount {
                mnemonic: "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("PzvfXhojANcckLxkydCHLEkhYZ8KxMXPSTEk8EXfOO0=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("WWg5J3N8TCvWfi8eh7DTtsJvVchbM4OA51JG8pNAuYo=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "uosmo")])
            },
            // osmo1829fz3hhsja3hruu04k9sa3gut6gas9mme5pa7
            ClaimAccount {
                mnemonic: "include monster floor galaxy skate second sister offer silver another upset mind frame into suit velvet lonely butter cousin side bridge answer logic pole",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("ikdFRp+5knN/Slwe9qXHnNfIPub6ILKYoZne6/5ss7I=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("WWg5J3N8TCvWfi8eh7DTtsJvVchbM4OA51JG8pNAuYo=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "uosmo")])
            },
            // osmo15fg0v8a4cztf2s2hchmm2rmcgungd2e27fu873
            ClaimAccount {
                mnemonic: "tide indoor kid review skin over try drive flower off inquiry winter summer what stick high memory atom hard deer kitchen must concert dizzy",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("MDgj78nEM3jFZyD6c/u+7xkpC0/CLHwQFxXTFFByjew=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("wMSBo52OnncjNj/bzlKpK4l+laX5a63KaAUDYcbkoyM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "uosmo")])
            },
            // osmo1s3u93f3v2wvncs5qdhv02c8vh6qhnqng877c2y
            ClaimAccount {
                mnemonic: "wrong join rifle knee myth woman layer actor question hockey fitness ignore bleak announce arrow crazy dish orbit divide melody pattern kiwi brown lawn",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("yWqhEHh9UUYSxGP/9yhiNVwgJlnmOZ6iMfeO8s76lsc=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("wMSBo52OnncjNj/bzlKpK4l+laX5a63KaAUDYcbkoyM=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "uosmo")])
            },
            // osmo1979s9gkuj9fyld4zzurgmh7hv3a30d3m64wyn2
            ClaimAccount {
                mnemonic: "boy casual file warfare family report embrace piece jewel garment loop device collect insane year flock swift open lobster infant antenna asset alcohol solid",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("XezlL3h1JQH5AEpfSNQ4FsCQbDk0djnCHvJbv4FsYJ8=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("BNTH3Nh0ebWjDHmuR2MUpe7pK5l0kTgUg1Hm8S3St/8=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000000, "uosmo")])
            },
            // osmo14rz2mwxa257rsag5du8vmfgnakmtr7ewjjut6k
            ClaimAccount {
                mnemonic: "tail praise mansion there pause cube poverty chalk dizzy dinner reveal electric mistake clever present pink blade cram coyote banana dog cargo cook someone",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("HHcflNV5O0y/6+aBgZdDE29byDkP1jxHGN3/Y9e04mY=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("BNTH3Nh0ebWjDHmuR2MUpe7pK5l0kTgUg1Hm8S3St/8=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000000, "uosmo")])
            },
            // osmo19j60xqsanl8fhv3773wrgjy3c7vvu2xs2t32kz
            ClaimAccount {
                mnemonic: "brown alley chunk iron stem they piece conduct near dirt poet truth clinic shallow pen above merit trophy gauge clerk excite evoke hour allow",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("Wphy16sLm9aCc1JGXWoUA7RGZwT/Xju53jyxAZqo7V0=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("T1MYqkDMc5Gd/6HCiu9LiKsR0xz+9If8CllBbHYW1/E=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000000, "uosmo")])
            },
            // osmo1qfy2pfpyeqxgy32a32vqwcalerhn8ctnl96y3k
            ClaimAccount {
                mnemonic: "humor exclude bulk trim fade sun moral mention topple keen nation convince desk tongue fish hill craft increase snack glass rural gate cheap mention",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XOoiQJz/7fqJTIdAp97hJ5uI6upzAeE7KyXKHMT1XQk=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("T1MYqkDMc5Gd/6HCiu9LiKsR0xz+9If8CllBbHYW1/E=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=").unwrap() },
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000900000, "uosmo")])
            },
            // osmo1yqxkfjugt457qy6slamn9zeyz4zv4upf24mr8g
            ClaimAccount {
                mnemonic: "perfect food future blush oak shrug tank under state illegal object awake erode poet tuition athlete answer sheriff say knee later fat dress visa",
                proofs: vec![
                    MerkleProof{ is_left_sibling: false, hash: base64::decode("WY5RgY0lZQKSJiDARUUpPGRt7KwGgcK4OUQ5g5vXHCU=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("GXr+vCP0gWPdYRS33SdNC2DWMmgo5EBK1SlH1mzY3zs=").unwrap() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000009000000, "uosmo")])
            },
            // osmo1y9ew9t7katv5pqpuh503lat7f4p6fkt9e2tsqd
            ClaimAccount {
                mnemonic: "smoke jealous man occur grief hat tobacco hospital fruit raise path primary secret budget wait police black panel resemble ten garden coach artefact engage",
                proofs: vec![
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("XEHPE7DZuYEWRnw3R6RfORvn33yykeoO40pJU7Zz68o=").unwrap() },
                    MerkleProof{ is_left_sibling: true, hash: base64::decode("GXr+vCP0gWPdYRS33SdNC2DWMmgo5EBK1SlH1mzY3zs=").unwrap() }
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

        // TODO: Execute AdminMsg::UpdateAdmin
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

        // Execute AdminMsg::UpdateMerkleRoot
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/merkle/100001.json#L3
        let merkle_root: &str = "iGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";
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
            println!("account {:?}", account.address().to_string());

            // Compose proof string
            let proof = serde_json_wasm::to_string(&claim_account.proofs)
                .expect("Failed to serialize proofs");

            let _ = wasm
                .execute(
                    contract.as_str(),
                    &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                        address: account.address(),
                        coins: claim_account.coins.clone(),
                        proof: proof.to_string(),
                    }),
                    &[],
                    &account,
                )
                .unwrap();

            // TODO: Assert bank send occurred
            let address_balace = bank
                .query_balance(&QueryBalanceRequest {
                    address: account.address().to_string(),
                    denom: "uosmo".to_string(),
                })
                .unwrap();
            assert_eq!(
                address_balace.balance.unwrap().amount,
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
