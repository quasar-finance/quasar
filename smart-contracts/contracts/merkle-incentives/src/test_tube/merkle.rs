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

    const MNEMONIC: &str = "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about";

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init();
        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // ToVerifies
        // osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt900000000uosmo
        // osmo1dplhdl5uch280hlcy3mdt207yf4vz3hkzpsysf9000000000uosmo
        // osmo1htv3nrpc69w5hnnxhh9pdpmfa7w5zvh3cst7cy90000000000uosmo
        // osmo1hr5u2ak5k6dydhpu5q348w5mw8fk69sj24x47l900000000000uosmo
        // osmo149pjhcf2sgwpmwqyknkkav4w2ywjjfc58egh6m9000000000000uosmo
        // osmo1n65qu2feqk8g0fcte2wtph3k3x55qsu720qyvx90000000000000uosmo
        // osmo18aduztzax34vtyhdqcarc455kn7xpldtdxn75s900000000000000uosmo
        // osmo1uctm0pn6fdwdlt48c6x9h29nwhg6jsr409q35m9000000000900000uosmo
        // osmo16xetz07p6v2laa7fqxfvtq2jx99g4yphaugp4490000000009000000uosmo
        // osmo1aemevl2sxymzmjxkaal4u8959qmxgpek4r7u6w900000000090000000uosmo

        // ClaimAccounts related to above ToVerifies
        // https://github.com/quasar-finance/merkle-incentives/blob/main/incentives/contracts/osmo1u4ppw4mxp00znxq5ll834dgr7ctd7jrp5hrzshch5ngfpwmp2fqsputgsx/fetch/100001.json#L3
        let claim_accounts: Vec<ClaimAccount> = vec![
            // osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("l0gejxqQNWS4+1AkGo0p2WZ8n4jnkSxGV7epueaKYfA=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Mw6ojCY/a3jbERFqj+XryESLjXkJn33z7FAT94H05ks=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("qs+0Uyuy8U3HqXkfFqSx3HEeVW4amvlZqeShtTGu2l0=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "uosmo")]),
            },
            // osmo1dplhdl5uch280hlcy3mdt207yf4vz3hkzpsysf
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("qQEesKAbniv8rw0b7HVsVDF7eQP25Q/8WBmeB1Vh2z0=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Mw6ojCY/a3jbERFqj+XryESLjXkJn33z7FAT94H05ks=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("qs+0Uyuy8U3HqXkfFqSx3HEeVW4amvlZqeShtTGu2l0=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "uosmo")]),
            },
            // osmo1htv3nrpc69w5hnnxhh9pdpmfa7w5zvh3cst7cy
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("WVV6NcAna6jUaNbIapvxuybuA80bjL1YJ5KpnyrgdT8=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("yYauEFyTLWgbk/glm4wHUCEj9JxQZL7R+hDhZ8uO++g=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("qs+0Uyuy8U3HqXkfFqSx3HEeVW4amvlZqeShtTGu2l0=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "uosmo")]),
            },
            // osmo1hr5u2ak5k6dydhpu5q348w5mw8fk69sj24x47l
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("WuGiVC2CM9OIxQpQ8re6W3POfefmqUyGMAbPH/b8FTk=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("yYauEFyTLWgbk/glm4wHUCEj9JxQZL7R+hDhZ8uO++g=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("qs+0Uyuy8U3HqXkfFqSx3HEeVW4amvlZqeShtTGu2l0=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "uosmo")]),
            },
            // osmo149pjhcf2sgwpmwqyknkkav4w2ywjjfc58egh6m
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("9k2pmVRpIxJsfO7oIw18bj0V7gE1KpG5yJHhLQc5WYs=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("ShZyIy1DWBLUKapgxqaQnLHVXdwmvdhfNAE+hRRI57U=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("+cnCHkFiAZCm2G37zToIJLnary2XrjbD/AkKypioqTA=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000000, "uosmo")]),
            },
            // osmo1n65qu2feqk8g0fcte2wtph3k3x55qsu720qyvx
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("mzrPZ96OIi3+FX74XXXtcTuMb/XMyzBNgHPW4a6lDz8=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("ShZyIy1DWBLUKapgxqaQnLHVXdwmvdhfNAE+hRRI57U=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("+cnCHkFiAZCm2G37zToIJLnary2XrjbD/AkKypioqTA=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000000, "uosmo")]),
            },
            // osmo18aduztzax34vtyhdqcarc455kn7xpldtdxn75s
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("jS3Pxqj6DMQ4ovpRdVEQxSl8YSiQMSbmdQDiStTffhk=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("w+o4owcrg9HmgDtnuaYfAyb6TDSwdW8wRLvnSjtDVyc=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("+cnCHkFiAZCm2G37zToIJLnary2XrjbD/AkKypioqTA=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000000, "uosmo")]),
            },
            // osmo1uctm0pn6fdwdlt48c6x9h29nwhg6jsr409q35m
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("yvfMozVEYVGQZzg6MMU5RC0Lx185NN8vxKMrU1pCqCs=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("w+o4owcrg9HmgDtnuaYfAyb6TDSwdW8wRLvnSjtDVyc=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("+cnCHkFiAZCm2G37zToIJLnary2XrjbD/AkKypioqTA=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("Pb8JPpb7JeUkM8k2y4ds5BOWoaqKkjKmtuepX/U6Cfo=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000900000, "uosmo")]),
            },
            // osmo16xetz07p6v2laa7fqxfvtq2jx99g4yphaugp44
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: false,
                        hash: base64::decode("DTb4sLi/k1mkTppOENb2UIMP7GuCsWFhXruy8WJ9JbE=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("px1Jmcuik72e1fQHfLWbVoE3GlD0Jy9KVShvyJDg3Ew=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(90000000009000000, "uosmo")]),
            },
            // osmo1aemevl2sxymzmjxkaal4u8959qmxgpek4r7u6w
            ClaimAccount {
                proofs: vec![
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("GyAndNBUQsi8hvFXE7SA50LdSnIHsKUM7UvVrgd3lDQ=").unwrap(),
                    },
                    MerkleProof {
                        is_left_sibling: true,
                        hash: base64::decode("px1Jmcuik72e1fQHfLWbVoE3GlD0Jy9KVShvyJDg3Ew=").unwrap(),
                    },
                ],
                coins: CoinVec::from(vec![Coin::new(900000000090000000, "uosmo")]),
            },
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

        // Create as many acocunt from mnemonic as ClaimAccounts
        let accounts = app
            .init_accounts_from_mnemonic(
                &[Coin::new(100_000_000_000_000_000_000, "uosmo")],
                MNEMONIC,
                claim_accounts.len() as u64,
            )
            .unwrap();

        // Execute IncentivesMsg::Claim
        for (index, claim_account) in claim_accounts.iter().enumerate() {
            println!(
                "addy {:?}",
                accounts.get(index).unwrap().address().to_string()
            );

            println!("claim_account.proofs {:?}", claim_account.proofs);
            let mut entries: Vec<Entry> = vec![];
            for proof in claim_account.clone().proofs {
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
                        address: accounts.get(index).unwrap().address(),
                        coins: claim_account.coins.clone(),
                        proof: proof_string.to_string(),
                    }),
                    &[],
                    &accounts.get(index).unwrap(),
                )
                .unwrap();

            // Assert bank send occurred
            let address_balance = bank
                .query_balance(&QueryBalanceRequest {
                    address: accounts.get(index).unwrap().address().to_string(),
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
