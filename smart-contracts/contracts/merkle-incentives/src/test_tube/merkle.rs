#[cfg(test)]
mod tests {
    use crate::{
        admin::execute::AdminExecuteMsg,
        incentives::{execute::IncentivesExecuteMsg, CoinVec},
        msg::ExecuteMsg,
        test_tube::initialize::initialize::default_init,
    };
    use bip39::Language;
    use cosmwasm_std::{Coin, Uint128};
    use osmosis_test_tube::cosmrs::bip32;
    use osmosis_test_tube::cosmrs::crypto::secp256k1::SigningKey;
    use osmosis_test_tube::{
        osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
        Account, Bank, FeeSetting, Module, SigningAccount, Wasm,
    };

    // TODO: Move those to state or somewhere else
    struct Proof {
        position: String,
        hex: String,
    }

    struct ClaimAccount {
        seed: String,
        proofs: Vec<Proof>,
        coins: CoinVec,
    }

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init();
        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // Test config

        let merkle_root: &str = "886a6d0b3db6b855a82319306aa473bf9c55e433271b3fa1267b713f6615b2ba";
        let claim_accounts: Vec<ClaimAccount> = vec![
            ClaimAccount {
                seed: "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about".to_string(),
                proofs: vec![
                    Proof{ position: "right".to_string(), hex: "3f3bdf5e1a2300d71c90bc64c9d0872c4921619f0ac4c5cf493124f045df38ed".to_string() },
                    Proof{ position: "right".to_string(), hex: "59683927737c4c2bd67e2f1e87b0d3b6c26f55c85b338380e75246f29340b98a".to_string() },
                    Proof{ position: "right".to_string(), hex: "9534cb53ffc6a44255e7753796c247ec16acd236b6632790745e0d6aa36b9c04".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "uosmo")])
            },
            ClaimAccount {
                seed: "include monster floor galaxy skate second sister offer silver another upset mind frame into suit velvet lonely butter cousin side bridge answer logic pole".to_string(),
                proofs: vec![
                    Proof{ position: "left".to_string(), hex: "8a4745469fb992737f4a5c1ef6a5c79cd7c83ee6fa20b298a199deebfe6cb3b2".to_string() },
                    Proof{ position: "right".to_string(), hex: "59683927737c4c2bd67e2f1e87b0d3b6c26f55c85b338380e75246f29340b98a".to_string() },
                    Proof{ position: "right".to_string(), hex: "9534cb53ffc6a44255e7753796c247ec16acd236b6632790745e0d6aa36b9c04".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "tide indoor kid review skin over try drive flower off inquiry winter summer what stick high memory atom hard deer kitchen must concert dizzy".to_string(),
                proofs: vec![
                    Proof{ position: "right".to_string(), hex: "303823efc9c43378c56720fa73fbbeef19290b4fc22c7c101715d31450728dec".to_string() },
                    Proof{ position: "left".to_string(), hex: "c0c481a39d8e9e7723363fdbce52a92b897e95a5f96badca68050361c6e4a323".to_string() },
                    Proof{ position: "right".to_string(), hex: "9534cb53ffc6a44255e7753796c247ec16acd236b6632790745e0d6aa36b9c04".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "wrong join rifle knee myth woman layer actor question hockey fitness ignore bleak announce arrow crazy dish orbit divide melody pattern kiwi brown lawn".to_string(),
                proofs: vec![
                    Proof{ position: "left".to_string(), hex: "c96aa110787d514612c463fff72862355c202659e6399ea231f78ef2cefa96c7".to_string() },
                    Proof{ position: "left".to_string(), hex: "c0c481a39d8e9e7723363fdbce52a92b897e95a5f96badca68050361c6e4a323".to_string() },
                    Proof{ position: "right".to_string(), hex: "9534cb53ffc6a44255e7753796c247ec16acd236b6632790745e0d6aa36b9c04".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "boy casual file warfare family report embrace piece jewel garment loop device collect insane year flock swift open lobster infant antenna asset alcohol solid".to_string(),
                proofs: vec![
                    Proof{ position: "right".to_string(), hex: "5dece52f78752501f9004a5f48d43816c0906c39347639c21ef25bbf816c609f".to_string() },
                    Proof{ position: "right".to_string(), hex: "04d4c7dcd87479b5a30c79ae476314a5eee92b99749138148351e6f12dd2b7ff".to_string() },
                    Proof{ position: "left".to_string(), hex: "5c976b32f2c5f8c42aadf675ff7c7527a5ceec87a621b25883937cfde653e0bb".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "tail praise mansion there pause cube poverty chalk dizzy dinner reveal electric mistake clever present pink blade cram coyote banana dog cargo cook someone".to_string(),
                proofs: vec![
                    Proof{ position: "left".to_string(), hex: "1c771f94d5793b4cbfebe681819743136f5bc8390fd63c4718ddff63d7b4e266".to_string() },
                    Proof{ position: "right".to_string(), hex: "04d4c7dcd87479b5a30c79ae476314a5eee92b99749138148351e6f12dd2b7ff".to_string() },
                    Proof{ position: "left".to_string(), hex: "5c976b32f2c5f8c42aadf675ff7c7527a5ceec87a621b25883937cfde653e0bb".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "brown alley chunk iron stem they piece conduct near dirt poet truth clinic shallow pen above merit trophy gauge clerk excite evoke hour allow".to_string(),
                proofs: vec![
                    Proof{ position: "right".to_string(), hex: "5a9872d7ab0b9bd6827352465d6a1403b4466704ff5e3bb9de3cb1019aa8ed5d".to_string() },
                    Proof{ position: "left".to_string(), hex: "4f5318aa40cc73919dffa1c28aef4b88ab11d31cfef487fc0a59416c7616d7f1".to_string() },
                    Proof{ position: "left".to_string(), hex: "5c976b32f2c5f8c42aadf675ff7c7527a5ceec87a621b25883937cfde653e0bb".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000000, "uosmo")])
            },
            ClaimAccount {
                seed: "humor exclude bulk trim fade sun moral mention topple keen nation convince desk tongue fish hill craft increase snack glass rural gate cheap mention".to_string(),
                proofs: vec![
                    Proof{ position: "left".to_string(), hex: "5cea22409cffedfa894c8740a7dee1279b88eaea7301e13b2b25ca1cc4f55d09".to_string() },
                    Proof{ position: "left".to_string(), hex: "4f5318aa40cc73919dffa1c28aef4b88ab11d31cfef487fc0a59416c7616d7f1".to_string() },
                    Proof{ position: "left".to_string(), hex: "5c976b32f2c5f8c42aadf675ff7c7527a5ceec87a621b25883937cfde653e0bb".to_string() },
                    Proof{ position: "right".to_string(), hex: "1c4bb7578df3d8b5206186fbc190b05eeab61ac62a9feddb240b8d0a37888969".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000900000, "uosmo")])
            },
            ClaimAccount {
                seed: "perfect food future blush oak shrug tank under state illegal object awake erode poet tuition athlete answer sheriff say knee later fat dress visa".to_string(),
                proofs: vec![
                    Proof{ position: "right".to_string(), hex: "598e51818d256502922620c04545293c646decac0681c2b8394439839bd71c25".to_string() },
                    Proof{ position: "left".to_string(), hex: "197afebc23f48163dd6114b7dd274d0b60d6326828e4404ad52947d66cd8df3b".to_string() }
                ],
                coins: CoinVec::from(vec![Coin::new(90000000009000000, "uosmo")])
            },
            ClaimAccount {
                seed: "smoke jealous man occur grief hat tobacco hospital fruit raise path primary secret budget wait police black panel resemble ten garden coach artefact engage".to_string(),
                proofs: vec![
                    Proof{ position: "left".to_string(), hex: "5c41cf13b0d9b98116467c3747a45f391be7df7cb291ea0ee34a4953b673ebca".to_string() },
                    Proof{ position: "left".to_string(), hex: "197afebc23f48163dd6114b7dd274d0b60d6326828e4404ad52947d66cd8df3b".to_string() }
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
                    amount: "1_000_000_000_000_000_000".to_string(), // 1_000_000_000_000.000000 OSMO (1T $OSMO)
                    denom: "uosmo".to_string(),
                }],
            },
            &admin,
        );

        // TODO: Assert initial balance on gauge contract

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

        // Execute AdminMsg::UpdateMerkleRoot
        // https://github.com/quasar-finance/merkle-incentives/blob/f45d842a2a6cf32d2b683f0893cae5bfaca9de3e/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/merkle/100001.json#L3
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
        // https://github.com/quasar-finance/merkle-incentives/blob/f45d842a2a6cf32d2b683f0893cae5bfaca9de3e/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/fetch/100001.json#L3

        for claim_account in claim_accounts {
            let signing_ccount: SigningAccount =
                init_account_from_mnemonic_phrase(claim_account.seed.as_str());

            // TODO: Simulate client combining proofs into a single one
            let proof = claim_account.proofs.get(0).unwrap().hex;

            let _ = wasm
                .execute(
                    contract.as_str(),
                    &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                        address: signing_ccount.address(),
                        coins: claim_account.coins,
                        proof,
                    }),
                    &[],
                    &signing_ccount,
                )
                .unwrap();

            // TODO: Assert bank send occurred
        }

        // TODO: Assert final balance on gauge contract
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
