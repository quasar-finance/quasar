#[cfg(test)]
mod tests {
    use crate::{
        admin::execute::AdminExecuteMsg,
        incentives::{execute::IncentivesExecuteMsg, CoinVec},
        msg::ExecuteMsg,
        state::{ClaimAccount, Proof},
        test_tube::initialize::initialize::default_init,
    };
    use bip39::Language;
    use cosmwasm_std::{Coin, Uint128};
    use osmosis_test_tube::cosmrs::crypto::secp256k1::SigningKey;
    use osmosis_test_tube::{
        cosmrs::bip32, osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest,
    };
    use osmosis_test_tube::{
        osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
        Account, Bank, FeeSetting, Module, SigningAccount, Wasm,
    };

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init();
        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // Test config

        let merkle_root: &str = "iGptCz22uFWoIxkwaqRzv5xV5DMnGz+hJntxP2YVsro=";
        let claim_accounts: Vec<ClaimAccount> = vec![
            ClaimAccount {
                seed: "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "false".to_string(), hash: "PzvfXhojANcckLxkydCHLEkhYZ8KxMXPSTEk8EXfOO0=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "WWg5J3N8TCvWfi8eh7DTtsJvVchbM4OA51JG8pNAuYo=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "uosmo")])
            },
            ClaimAccount {
                seed: "include monster floor galaxy skate second sister offer silver another upset mind frame into suit velvet lonely butter cousin side bridge answer logic pole".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "true".to_string(), hash: "ikdFRp+5knN/Slwe9qXHnNfIPub6ILKYoZne6/5ss7I=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "WWg5J3N8TCvWfi8eh7DTtsJvVchbM4OA51JG8pNAuYo=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "tide indoor kid review skin over try drive flower off inquiry winter summer what stick high memory atom hard deer kitchen must concert dizzy".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "false".to_string(), hash: "MDgj78nEM3jFZyD6c/u+7xkpC0/CLHwQFxXTFFByjew=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "wMSBo52OnncjNj/bzlKpK4l+laX5a63KaAUDYcbkoyM=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "wrong join rifle knee myth woman layer actor question hockey fitness ignore bleak announce arrow crazy dish orbit divide melody pattern kiwi brown lawn".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "true".to_string(), hash: "yWqhEHh9UUYSxGP/9yhiNVwgJlnmOZ6iMfeO8s76lsc=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "wMSBo52OnncjNj/bzlKpK4l+laX5a63KaAUDYcbkoyM=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "lTTLU//GpEJV53U3lsJH7Bas0ja2YyeQdF4NaqNrnAQ=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "boy casual file warfare family report embrace piece jewel garment loop device collect insane year flock swift open lobster infant antenna asset alcohol solid".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "false".to_string(), hash: "XezlL3h1JQH5AEpfSNQ4FsCQbDk0djnCHvJbv4FsYJ8=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "BNTH3Nh0ebWjDHmuR2MUpe7pK5l0kTgUg1Hm8S3St/8=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "tail praise mansion there pause cube poverty chalk dizzy dinner reveal electric mistake clever present pink blade cram coyote banana dog cargo cook someone".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "true".to_string(), hash: "HHcflNV5O0y/6+aBgZdDE29byDkP1jxHGN3/Y9e04mY=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "BNTH3Nh0ebWjDHmuR2MUpe7pK5l0kTgUg1Hm8S3St/8=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "brown alley chunk iron stem they piece conduct near dirt poet truth clinic shallow pen above merit trophy gauge clerk excite evoke hour allow".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "false".to_string(), hash: "Wphy16sLm9aCc1JGXWoUA7RGZwT/Xju53jyxAZqo7V0=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "T1MYqkDMc5Gd/6HCiu9LiKsR0xz+9If8CllBbHYW1/E=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "humor exclude bulk trim fade sun moral mention topple keen nation convince desk tongue fish hill craft increase snack glass rural gate cheap mention".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "true".to_string(), hash: "XOoiQJz/7fqJTIdAp97hJ5uI6upzAeE7KyXKHMT1XQk=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "T1MYqkDMc5Gd/6HCiu9LiKsR0xz+9If8CllBbHYW1/E=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "XJdrMvLF+MQqrfZ1/3x1J6XO7IemIbJYg5N8/eZT4Ls=".to_string() },
                    Proof{ is_left_sibling: "false".to_string(), hash: "HEu3V43z2LUgYYb7wZCwXuq2GsYqn+3bJAuNCjeIiWk=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000900000, "uosmo")])
            },
            ClaimAccount {
                seed: "perfect food future blush oak shrug tank under state illegal object awake erode poet tuition athlete answer sheriff say knee later fat dress visa".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "false".to_string(), hash: "WY5RgY0lZQKSJiDARUUpPGRt7KwGgcK4OUQ5g5vXHCU=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "GXr+vCP0gWPdYRS33SdNC2DWMmgo5EBK1SlH1mzY3zs=".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000009000000, "uosmo")])
            },
            ClaimAccount {
                seed: "smoke jealous man occur grief hat tobacco hospital fruit raise path primary secret budget wait police black panel resemble ten garden coach artefact engage".to_string(),
                proofs: vec![
                    Proof{ is_left_sibling: "true".to_string(), hash: "XEHPE7DZuYEWRnw3R6RfORvn33yykeoO40pJU7Zz68o=".to_string() },
                    Proof{ is_left_sibling: "true".to_string(), hash: "GXr+vCP0gWPdYRS33SdNC2DWMmgo5EBK1SlH1mzY3zs=".to_string() }
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
        // let new_admin = app
        //     .init_account(&[Coin::new(1_000_000_000, "uosmo")])
        //     .unwrap();
        // let _ = wasm
        //     .execute(
        //         contract.as_str(),
        //         &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateAdmin {
        //             new_admin: new_admin.address(),
        //         }),
        //         &[],
        //         &admin,
        //     )
        //     .unwrap();

        // TODO: Assert admin changed and queriable

        // Execute AdminMsg::UpdateMerkleRoot
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/merkle/100001.json#L3
        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateMerkleRoot {
                    new_root: merkle_root.to_string(),
                }),
                &[],
                &admin,
            )
            .unwrap();

        // TODO: Assert merkle root changed and queriable

        // Execute IncentivesMsg::Claim
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/fetch/100001.json#L3
        for claim_account in claim_accounts {
            let signing_ccount: SigningAccount =
                init_account_from_mnemonic_phrase(claim_account.seed.as_str());

            let proof = serde_json_wasm::to_string(&claim_account.proofs)
                .expect("Failed to serialize proofs");

            let _ = wasm
                .execute(
                    contract.as_str(),
                    &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                        address: signing_ccount.address(),
                        coins: claim_account.coins.clone(),
                        proof: proof.to_string(),
                    }),
                    &[],
                    &signing_ccount,
                )
                .unwrap();

            // TODO: Assert bank send occurred
            let address_balace = bank
                .query_balance(&QueryBalanceRequest {
                    address: signing_ccount.address().to_string(),
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
        let contract_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: contract.to_string(),
                denom: "uosmo".to_string(),
            })
            .unwrap();
        assert_eq!(
            contract_balance.balance.unwrap().amount,
            "100000".to_string()
        ); // Due to precision loss
    }

    fn init_account_from_mnemonic_phrase(mnemonic_phrase: &str) -> SigningAccount {
        let mnemonic = bip39::Mnemonic::from_phrase(mnemonic_phrase, Language::English).unwrap();

        let seed = bip39::Seed::new(&mnemonic, "");
        let derivation_path = "m/44'/118'/0'/0/0"
            .parse::<bip32::DerivationPath>()
            .unwrap();
        let signing_key = SigningKey::derive_from_path(seed, &derivation_path).unwrap();
        let signing_account = SigningAccount::new(
            "osmo".to_string(),
            signing_key,
            FeeSetting::Auto {
                gas_price: Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::new(1000000u128),
                },
                gas_adjustment: 1.3 as f64,
            },
        );

        signing_account
    }
}
