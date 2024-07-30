#![cfg(feature = "test-tube")]

use base64::{engine::general_purpose::STANDARD, Engine};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::KeyDeserialize;
use merkle_incentives::msg::InstantiateMsg;
use merkle_incentives::{
    admin::execute::AdminExecuteMsg,
    incentives::{execute::IncentivesExecuteMsg, query::IncentivesQueryMsg, CoinVec},
    msg::{ExecuteMsg, QueryMsg},
    state::ClaimAccount,
};
use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use osmosis_test_tube::{
    osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
    Account, Bank, Module, OsmosisTestApp, SigningAccount, Wasm,
};
use rs_merkle::{
    algorithms::{self, Sha256},
    Hasher, MerkleProof, MerkleTree,
};

fn default_init(gauge_coins: Vec<Coin>) -> (OsmosisTestApp, Addr, SigningAccount) {
    init_test_contract(
        "./test-tube-build/wasm32-unknown-unknown/release/merkle_incentives.wasm",
        gauge_coins,
    )
}

fn init_test_contract(
    filename: &str,
    gauge_coins: Vec<Coin>,
) -> (OsmosisTestApp, Addr, SigningAccount) {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    // Ensure uosmo is always included by checking and adding if necessary
    let mut coins_with_uosmo = gauge_coins.clone();
    if !gauge_coins.iter().any(|coin| coin.denom == "uosmo") {
        coins_with_uosmo.push(Coin::new(100_000_000_000_000_000_000, "uosmo"));
    }

    let admin = app.init_account(&coins_with_uosmo).unwrap();

    let wasm_byte_code = std::fs::read(filename).unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, &admin)
        .unwrap()
        .data
        .code_id;

    let contract = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {},
            Some(admin.address().as_str()),
            Some("merkle-incentives"),
            &[],
            &admin,
        )
        .unwrap();

    (app, Addr::unchecked(contract.data.address), admin)
}

#[test]
fn merkle_complete_cycle_works() {
    let (app, contract, admin) = default_init(vec![Coin {
        denom: "ugauge".to_string(),
        amount: Uint128::new(1000000000000000000u128),
    }]);
    let bank = Bank::new(&app);
    let wasm = Wasm::new(&app);

    let accounts = app
        .init_accounts(&[Coin::new(100_000_000_000, "uosmo")], 10)
        .unwrap();

    let leaves_str = vec![
        format!("{}900000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}9000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}90000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}900000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}9000000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}90000000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!("{}900000000000000ugauge", accounts[0].address().to_string()).to_string(),
        format!(
            "{}9000000000900000ugauge",
            accounts[0].address().to_string()
        )
        .to_string(),
        format!(
            "{}90000000009000000ugauge",
            accounts[0].address().to_string()
        )
        .to_string(),
        format!(
            "{}900000000090000000ugauge",
            accounts[0].address().to_string()
        )
        .to_string(),
    ];

    let leaves = leaves_str
        .iter()
        .map(|x| algorithms::Sha256::hash(x.as_bytes()))
        .collect::<Vec<[u8; 32]>>();

    let merkle_tree: MerkleTree<algorithms::Sha256> = rs_merkle::MerkleTree::from_leaves(&leaves);

    let indices_to_prove = vec![0];
    let leaves_to_prove = leaves.get(0..1).unwrap();
    let merkle_proof = merkle_tree.proof(&indices_to_prove);

    let merkle_root = merkle_tree.root().unwrap();

    let proof_bytes = merkle_proof.to_bytes();

    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    assert!(proof.verify(
        merkle_root,
        &indices_to_prove,
        leaves_to_prove,
        leaves.len()
    ));

    let claim_accounts: Vec<ClaimAccount> = vec![ClaimAccount {
        proof: merkle_proof
            .proof_hashes()
            .iter()
            .map(|x| Vec::from_slice(x).unwrap())
            .collect(),
        coins: CoinVec::from(vec![Coin::new(900000000, "ugauge")]),
    }];

    let one_trillion_str = "1000000000000000000".to_string();
    let _ = bank.send(
        MsgSend {
            from_address: admin.address().to_string(),
            to_address: contract.to_string(),
            amount: vec![OsmoCoin {
                amount: one_trillion_str.clone(),
                denom: "ugauge".to_string(),
            }],
        },
        &admin,
    );

    let contract_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract.to_string(),
            denom: "ugauge".to_string(),
        })
        .unwrap();
    assert_eq!(contract_balance.balance.unwrap().amount, one_trillion_str);

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

    let binding = STANDARD.encode(merkle_tree.root().unwrap());
    let merkle_root: &str = binding.as_str();
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

    for (index, claim_account) in claim_accounts.iter().enumerate() {
        let mut proof_hashes: Vec<[u8; 32]> = Vec::new();
        for proof in &claim_account.proof {
            if proof.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&proof);
                proof_hashes.push(arr);
            } else {
                eprintln!("Error: Hash is not 32 bytes.");
            }
        }

        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                    address: accounts.get(index).unwrap().address().to_string(),
                    coins: claim_account.coins.clone(),
                    proof_hashes,
                    leaf_index: index,
                    total_leaves_count: 10usize,
                }),
                &[],
                &accounts.get(index).unwrap(),
            )
            .unwrap();

        let address_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: accounts.get(index).unwrap().address().to_string(),
                denom: "ugauge".to_string(),
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
            denom: "ugauge".to_string(),
        })
        .unwrap();
    assert_eq!(
        contract_balance.balance.unwrap().amount,
        "999999999100000000".to_string()
    );
}

#[test]
fn merkle_complete_cycle_works_mainnet_data() {
    let (app, contract, admin) = default_init(vec![
        Coin {
            denom: "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo".to_string(),
            amount: Uint128::new(300000000u128),
        },
        Coin {
            denom: "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588"
                .to_string(),
            amount: Uint128::new(27500000u128),
        },
    ]);
    let bank = Bank::new(&app);
    let wasm = Wasm::new(&app);

    let _ = bank.send(
        MsgSend {
            from_address: admin.address().to_string(),
            to_address: contract.to_string(),
            amount: vec![OsmoCoin {
                amount: "300000000".to_string(),
                denom: "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo".to_string(),
            }],
        },
        &admin,
    );
    let _ = bank.send(
        MsgSend {
            from_address: admin.address().to_string(),
            to_address: contract.to_string(),
            amount: vec![OsmoCoin {
                amount: "27500000".to_string(),
                denom: "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588"
                    .to_string(),
            }],
        },
        &admin,
    );

    let contract_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract.to_string(),
            denom: "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo".to_string(),
        })
        .unwrap();
    assert_eq!(
        contract_balance.balance.unwrap().amount,
        "300000000".to_string()
    );
    let contract_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract.to_string(),
            denom: "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588"
                .to_string(),
        })
        .unwrap();
    assert_eq!(
        contract_balance.balance.unwrap().amount,
        "27500000".to_string()
    );

    let merkle_root: &str = "M1aYKj1y+ClZeeyNzfznnpJIgWhuPvpExosYK0G5nFU=";
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

    let proofs: Vec<(&str, CoinVec, Vec<Vec<u8>>)> = vec![
        (
            "osmo1t9g70h7su684gggcl0g7e2l3r3gjk9x4eppvxv",
            CoinVec::from(vec![
                Coin::new(
                    659745,
                    "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo",
                ),
                Coin::new(
                    60426,
                    "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588",
                ),
            ]),
            vec![
                STANDARD
                    .decode("j3zyEtaQ88vnMBN4Wbb2vssWy3LEGJerxlz2xyHcWEY=")
                    .unwrap(),
                STANDARD
                    .decode("L7EOyHK/x+/x/Pc0osavxYd8qqb6bg9W5E2ppXOxHKc=")
                    .unwrap(),
            ],
        ),
        (
            "osmo1dx6rd4j4qg8tgzzm2l0a0yl8520xk8ukdfs65d",
            CoinVec::from(vec![
                Coin::new(
                    146212740,
                    "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo",
                ),
                Coin::new(
                    13378537,
                    "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588",
                ),
            ]),
            vec![
                STANDARD
                    .decode("VJEFLHCJcLsip2zZf/C/VYZCz8x8ioqLOSiZHCizpj4=")
                    .unwrap(),
                STANDARD
                    .decode("L7EOyHK/x+/x/Pc0osavxYd8qqb6bg9W5E2ppXOxHKc=")
                    .unwrap(),
            ],
        ),
        (
            "osmo16nsxukkff43y703xzj4p7mcg9z7enuher6h4t4",
            CoinVec::from(vec![
                Coin::new(
                    11477154,
                    "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo",
                ),
                Coin::new(
                    1050442,
                    "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588",
                ),
            ]),
            vec![
                STANDARD
                    .decode("bXHudyZ1T8GL1JwCp7m2pWiNfA8ixD8S9POOelU0qZI=")
                    .unwrap(),
                STANDARD
                    .decode("DZuG8wjPum3sMPU98Ld4WvSXcHOMz8v32twL1/OOLyQ=")
                    .unwrap(),
            ],
        ),
        (
            "osmo1c7m4gmauthgs5rhacgvfxkp6z4l38sclu4gtaw",
            CoinVec::from(vec![
                Coin::new(
                    35479062,
                    "factory/osmo1nz7qdp7eg30sr959wvrwn9j9370h4xt6ttm0h3/ussosmo",
                ),
                Coin::new(
                    3247592,
                    "ibc/206DE6A2D1882D91A12F01C8BD85A33C9673C6A39761D36CF736436D6EF4C588",
                ),
            ]),
            vec![
                STANDARD
                    .decode("MnF8oGUkUxO9Z1J+E9CnOe0fPEzzQZ85X0W77btlINo=")
                    .unwrap(),
                STANDARD
                    .decode("DZuG8wjPum3sMPU98Ld4WvSXcHOMz8v32twL1/OOLyQ=")
                    .unwrap(),
            ],
        ),
    ];
    let mut proof_hashes: Vec<(&str, CoinVec, Vec<[u8; 32]>)> = Vec::new();

    for (address, coins, proofs) in &proofs {
        let mut user_proofs: Vec<[u8; 32]> = Vec::new();
        for proof in proofs {
            if proof.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(proof);
                user_proofs.push(arr);
            } else {
                eprintln!("Error: Hash is not 32 bytes.");
            }
        }
        if user_proofs.len() == 2 {
            proof_hashes.push((*address, coins.clone(), user_proofs));
        } else {
            eprintln!("Error: Expected two proofs for address {}", address);
        }
    }

    for (index, (address, coins, user_proofs)) in proof_hashes.iter().enumerate() {
        let msg = ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
            address: address.to_string(),
            coins: coins.clone(),
            proof_hashes: user_proofs.clone(),
            leaf_index: index,
            total_leaves_count: proof_hashes.len(),
        });
        let _ = wasm.execute(contract.as_str(), &msg, &[], &admin).unwrap();

        let _: Option<CoinVec> = wasm
            .query(
                contract.as_str(),
                &QueryMsg::IncentivesQuery(IncentivesQueryMsg::ClaimedIncentives {
                    address: address.to_string(),
                }),
            )
            .unwrap();
    }
}
