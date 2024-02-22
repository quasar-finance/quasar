use super::CoinVec;
use crate::{
    error::ContractError,
    state::{CLAIMED_INCENTIVES, MERKLE_ROOT},
};
use cosmwasm_std::{Addr, Deps};
use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleProof};

pub fn is_valid_claim(
    deps: Deps,
    address: Addr,
    coins: &CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
) -> Result<CoinVec, ContractError> {
    let merkle_root = MERKLE_ROOT.load(deps.storage)?;

    // the format of this will look like "addr1000utokena2000utokenb"
    let claim_string = format!("{}{}", address.to_string(), coins.to_string());

    verify_proof(
        &merkle_root,
        proof_hashes,
        &[leaf_index],
        total_leaves_count,
        &claim_string,
    )?;

    let claimed_amount = CLAIMED_INCENTIVES
        .load(deps.storage, address.clone())
        .unwrap_or(CoinVec::new());

    if &claimed_amount >= coins {
        return Err(ContractError::IncentivesAlreadyClaimed {});
    }

    // subtract the current claim from all time claims
    let claim_amount = coins.checked_sub(claimed_amount)?;

    Ok(claim_amount)
}

pub fn verify_proof(
    merkle_root: &str,
    proof_hashes: Vec<[u8; 32]>,
    leaf_indices: &[usize],
    total_leaves_count: usize,
    to_verify: &str,
) -> Result<(), ContractError> {
    let root_hash = base64::decode(merkle_root).unwrap();
    let to_verify_hash = Sha256::hash(to_verify.as_bytes());

    let proof = MerkleProof::<Sha256>::new(proof_hashes);

    let is_valid = proof.verify(
        root_hash.try_into().unwrap(),
        leaf_indices,
        &[to_verify_hash],
        total_leaves_count,
    );

    if is_valid {
        Ok(())
    } else {
        Err(ContractError::FailedVerifyProof {})
    }
}

#[cfg(test)]
mod tests {
    use crate::{incentives::helpers::verify_proof, ContractError};

    const MERKLE_ROOT_STRING: &str = "0hGvbH+l9pdPgOmJY6wZuwjsrvtPsuslgTURavrUP6I=";
    const MERKLE_ROOT_INVALID_STRING: &str = "INVALIDROOTRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI=";
    const CLAIM_PROOF_STRING: &str = "osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt900000000ugauge";
    const CLAIM_PROOF_INVALID_STRING: &str =
        "osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt999999999ugauge";

    #[test]
    fn test_verify_success() {
        let proof = vec![
            base64::decode("R6J/QIhrqN4KxMa7ZhaCm/6J7ibT7HHcw9KKRV4ML0k=").unwrap(),
            base64::decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=").unwrap(),
            base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
            base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
        ];
        let mut proof_hashes: Vec<[u8; 32]> = Vec::new();
        for proof in &proof {
            if proof.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&proof);
                proof_hashes.push(arr);
            } else {
                eprintln!("Error: Hash is not 32 bytes.");
            }
        }

        verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            proof_hashes,
            &[0usize],
            10usize,
            CLAIM_PROOF_STRING,
        )
        .unwrap();
    }

    #[test]
    fn test_verify_bad_root() {
        let proof = vec![
            base64::decode("R6J/QIhrqN4KxMa7ZhaCm/6J7ibT7HHcw9KKRV4ML0k=").unwrap(),
            base64::decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=").unwrap(),
            base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
            base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
        ];
        let mut proof_hashes: Vec<[u8; 32]> = Vec::new();
        for proof in &proof {
            if proof.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&proof);
                proof_hashes.push(arr);
            } else {
                eprintln!("Error: Hash is not 32 bytes.");
            }
        }

        let result = verify_proof(
            &MERKLE_ROOT_INVALID_STRING.to_string(),
            proof_hashes,
            &[0usize],
            10usize,
            CLAIM_PROOF_STRING,
        );

        if let Err(ContractError::FailedVerifyProof {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }

    #[test]
    fn test_verify_bad_claim() {
        let proof = vec![
            base64::decode("R6J/QIhrqN4KxMa7ZhaCm/6J7ibT7HHcw9KKRV4ML0k=").unwrap(),
            base64::decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=").unwrap(),
            base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
            base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
        ];
        let mut proof_hashes: Vec<[u8; 32]> = Vec::new();
        for proof in &proof {
            if proof.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&proof);
                proof_hashes.push(arr);
            } else {
                eprintln!("Error: Hash is not 32 bytes.");
            }
        }

        let result = verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            proof_hashes,
            &[0usize],
            10usize,
            CLAIM_PROOF_INVALID_STRING,
        );

        if let Err(ContractError::FailedVerifyProof {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }

    // #[test]
    // fn test_is_valid_claim_true() {
    //     // this test is taken directly from the testdata. See the README.md of this contract
    //     let mut deps = mock_dependencies();

    //     let claim_coins = vec![
    //         // notice these are not alphabetically sorted
    //         Coin {
    //             denom: "uxyz".to_string(),
    //             amount: Uint128::from(1u128),
    //         },
    //         Coin {
    //             denom: "uosmo".to_string(),
    //             amount: Uint128::from(7u128),
    //         },
    //     ];

    //     MERKLE_ROOT
    //         .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
    //         .unwrap();

    //     let result = is_valid_claim(
    //         deps.as_ref(),
    //         Addr::unchecked(USER_ADDRESS),
    //         &claim_coins.clone().into(),
    //         USER_MERKLE_PROOF.to_string(),
    //     );

    //     assert_eq!(result.unwrap(), claim_coins.into());
    // }

    // #[test]
    // fn test_is_valid_claim_some_already_claimed() {
    //     // this test is taken directly from the testdata. See the README.md of this contract
    //     let mut deps = mock_dependencies();

    //     let claim_coins = vec![
    //         // notice these are not alphabetically sorted
    //         Coin {
    //             denom: "uxyz".to_string(),
    //             amount: Uint128::from(1u128),
    //         },
    //         Coin {
    //             denom: "uosmo".to_string(),
    //             amount: Uint128::from(7u128),
    //         },
    //     ];

    //     MERKLE_ROOT
    //         .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
    //         .unwrap();

    //     CLAIMED_INCENTIVES
    //         .save(
    //             deps.as_mut().storage,
    //             Addr::unchecked(USER_ADDRESS),
    //             &CoinVec(vec![Coin {
    //                 denom: "uosmo".to_string(),
    //                 amount: Uint128::from(3u128),
    //             }]),
    //         )
    //         .unwrap();

    //     let result = is_valid_claim(
    //         deps.as_ref(),
    //         &claim_coins.clone().into(),
    //         USER_MERKLE_PROOF.to_string(),
    //     );

    //     let expected_claim = vec![
    //         // notice these are not alphabetically sorted
    //         Coin {
    //             denom: "uxyz".to_string(),
    //             amount: Uint128::from(1u128),
    //         },
    //         Coin {
    //             denom: "uosmo".to_string(),
    //             amount: Uint128::from(4u128),
    //         },
    //     ];
    //     assert_eq!(result.unwrap(), expected_claim.into());
    // }

    // #[test]
    // fn test_is_valid_claim_all_already_claimed() {
    //     // this test is taken directly from the testdata. See the README.md of this contract
    //     let mut deps = mock_dependencies();

    //     let claim_coins = vec![
    //         // notice these are not alphabetically sorted
    //         Coin {
    //             denom: "uxyz".to_string(),
    //             amount: Uint128::from(1u128),
    //         },
    //         Coin {
    //             denom: "uosmo".to_string(),
    //             amount: Uint128::from(7u128),
    //         },
    //     ];

    //     MERKLE_ROOT
    //         .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
    //         .unwrap();

    //     CLAIMED_INCENTIVES
    //         .save(
    //             deps.as_mut().storage,
    //             Addr::unchecked(USER_ADDRESS),
    //             &CoinVec(vec![
    //                 Coin {
    //                     denom: "uosmo".to_string(),
    //                     amount: Uint128::from(7u128),
    //                 },
    //                 Coin {
    //                     denom: "uxyz".to_string(),
    //                     amount: Uint128::from(1u128),
    //                 },
    //             ]),
    //         )
    //         .unwrap();

    //     let result = super::is_valid_claim(
    //         deps.as_ref(),
    //         Addr::unchecked(USER_ADDRESS),
    //         &claim_coins.clone().into(),
    //         USER_MERKLE_PROOF.to_string(),
    //     );
    //     if let Err(ContractError::IncentivesAlreadyClaimed {}) = result {
    //         assert!(true); // expected
    //     } else {
    //         panic!("unexpected result");
    //     }
    // }

    // #[test]
    // fn test_is_valid_claim_with_bad_claim_amount() {
    //     // this test is taken directly from the testdata. See the README.md of this contract
    //     let mut deps = mock_dependencies();

    //     let claim_coins = vec![
    //         // notice these are not alphabetically sorted
    //         Coin {
    //             denom: "uxyz".to_string(),
    //             amount: Uint128::from(1u128),
    //         },
    //         Coin {
    //             denom: "uosmo".to_string(),
    //             amount: Uint128::from(8u128), // trying to claim too much of uosmo
    //         },
    //     ];

    //     MERKLE_ROOT
    //         .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
    //         .unwrap();

    //     CLAIMED_INCENTIVES
    //         .save(
    //             deps.as_mut().storage,
    //             Addr::unchecked(USER_ADDRESS),
    //             &CoinVec(vec![
    //                 Coin {
    //                     denom: "uosmo".to_string(),
    //                     amount: Uint128::from(7u128),
    //                 },
    //                 Coin {
    //                     denom: "uxyz".to_string(),
    //                     amount: Uint128::from(1u128),
    //                 },
    //             ]),
    //         )
    //         .unwrap();

    //     let result = super::is_valid_claim(
    //         deps.as_ref(),
    //         Addr::unchecked(USER_ADDRESS),
    //         &claim_coins.clone().into(),
    //         USER_MERKLE_PROOF.to_string(),
    //     );
    //     if let Err(ContractError::FailedVerifyProof {}) = result {
    //         assert!(true); // expected
    //     } else {
    //         panic!("unexpected result");
    //     }
    // }
}
