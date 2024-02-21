use super::CoinVec;
use crate::{
    error::ContractError,
    state::{CLAIMED_INCENTIVES, MERKLE_ROOT},
};
use cosmwasm_std::{Addr, Deps};
use merkle::{hash::Hash, proof::Proof};

pub fn is_valid_claim(
    deps: Deps,
    address: Addr,
    coins: &CoinVec,
    proof: String,
) -> Result<CoinVec, ContractError> {
    // the format of this will look like "addr1000utokena2000utokenb"
    let claim = format!("{}{}", address.to_string(), coins.to_string());

    let merkle_root = MERKLE_ROOT.load(deps.storage)?;
    deps.api
        .debug(format!("merkle_root {:?}", merkle_root).as_str());

    verify_proof(&merkle_root, &proof, &claim)?;

    let user_claimed = CLAIMED_INCENTIVES
        .load(deps.storage, address.clone())
        .unwrap_or(CoinVec::new());

    if &user_claimed >= coins {
        return Err(ContractError::IncentivesAlreadyClaimed {});
    }

    // subtract the current claim from all time claims
    let this_claim = coins.checked_sub(user_claimed)?;

    Ok(this_claim)
}

pub fn verify_proof(merkle_root: &str, proof: &str, to_verify: &str) -> Result<(), ContractError> {
    let proof: Proof = serde_json_wasm::from_str(proof).unwrap();
    let root = match base64::decode(merkle_root) {
        Ok(f) => f,
        Err(e) => {
            return Err(ContractError::FailedToDecodeRoot {
                root: e.to_string(),
            })
        }
    };

    let root_hash = Hash::from(root);

    if !proof.verify(&to_verify, &root_hash) {
        return Err(ContractError::FailedVerifyProof {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr, Coin, Uint128};

    use crate::{
        incentives::CoinVec,
        state::{CLAIMED_INCENTIVES, MERKLE_ROOT},
        ContractError,
    };

    use super::verify_proof;

    const MERKLE_ROOT_STRING: &str = "rZh9kBgioPQRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI=";
    const USER_MERKLE_PROOF: &str = r#"[{"is_left_sibling":false,"hash":[100,110,108,104,75,52,65,71,97,67,66,74,49,111,98,50,43,108,51,43,115,97,106,74,57,102,56,57,103,89,86,69,81,107,85,47,78,98,73,119,66,105,115,61]},{"is_left_sibling":false,"hash":[80,101,119,71,108,73,79,97,114,52,98,49,89,122,69,111,90,47,74,105,99,115,105,50,84,74,122,100,98,54,80,72,103,71,52,110,97,66,85,105,97,75,111,61]},{"is_left_sibling":true,"hash":[98,103,119,115,113,65,118,107,79,99,79,115,48,81,85,80,110,70,115,81,76,107,108,119,71,115,68,102,50,111,106,98,50,116,67,107,49,81,53,49,69,112,73,61]},{"is_left_sibling":true,"hash":[122,99,119,55,111,117,82,71,68,112,57,79,72,89,56,105,77,47,88,122,87,80,119,70,104,70,88,52,53,66,120,80,74,98,70,103,98,82,69,122,82,103,56,61]},{"is_left_sibling":false,"hash":[77,113,116,72,72,81,43,48,109,54,115,55,82,113,97,84,100,121,122,56,69,74,65,54,51,97,81,89,83,119,112,109,100,119,122,99,111,90,80,105,122,50,69,61]}]"#;
    const USER_ADDRESS: &str = "osmo10004ufcv2aln3vl8defyk9agv5kacrzpkyw5p4";
    const TO_VERIFY: &str = "osmo10004ufcv2aln3vl8defyk9agv5kacrzpkyw5p47uosmo1uxyz";

    #[test]
    fn test_verify_success() {
        // this test is taken directly from the testdata. See the README.md of this contract
        verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            USER_MERKLE_PROOF,
            TO_VERIFY,
        )
        .unwrap();
    }

    #[test]
    fn test_verify_err() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let invalid_merkle_root = "INVALIDROOTRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI=";

        let result = verify_proof(
            &invalid_merkle_root.to_string(),
            USER_MERKLE_PROOF,
            TO_VERIFY,
        );

        if let Err(ContractError::FailedVerifyProof {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }

    #[test]
    fn test_verify_bad_claim() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let to_verify_invalid = "osmo10004ufcv2aln3vl8defyk9agv5kacrzpkyw5p47uosmo10uxyz"; // 9 extra uxyz attempted

        let result = verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            USER_MERKLE_PROOF,
            to_verify_invalid,
        );
        println!("Result: {:?}", result);
        if let Err(ContractError::FailedVerifyProof {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }

    #[test]
    fn test_is_valid_claim_true() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let mut deps = mock_dependencies();

        let claim_coins = vec![
            // notice these are not alphabetically sorted
            Coin {
                denom: "uxyz".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(7u128),
            },
        ];

        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();

        let result = super::is_valid_claim(
            deps.as_ref(),
            Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            USER_MERKLE_PROOF.to_string(),
        );

        assert_eq!(result.unwrap(), claim_coins.into());
    }

    #[test]
    fn test_is_valid_claim_some_already_claimed() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let mut deps = mock_dependencies();

        let claim_coins = vec![
            // notice these are not alphabetically sorted
            Coin {
                denom: "uxyz".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(7u128),
            },
        ];

        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();

        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::from(3u128),
                }]),
            )
            .unwrap();

        let result = super::is_valid_claim(
            deps.as_ref(),
            Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            USER_MERKLE_PROOF.to_string(),
        );

        let expected_claim = vec![
            // notice these are not alphabetically sorted
            Coin {
                denom: "uxyz".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(4u128),
            },
        ];
        assert_eq!(result.unwrap(), expected_claim.into());
    }

    #[test]
    fn test_is_valid_claim_all_already_claimed() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let mut deps = mock_dependencies();

        let claim_coins = vec![
            // notice these are not alphabetically sorted
            Coin {
                denom: "uxyz".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(7u128),
            },
        ];

        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();

        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![
                    Coin {
                        denom: "uosmo".to_string(),
                        amount: Uint128::from(7u128),
                    },
                    Coin {
                        denom: "uxyz".to_string(),
                        amount: Uint128::from(1u128),
                    },
                ]),
            )
            .unwrap();

        let result = super::is_valid_claim(
            deps.as_ref(),
            Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            USER_MERKLE_PROOF.to_string(),
        );
        if let Err(ContractError::IncentivesAlreadyClaimed {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }

    #[test]
    fn test_is_valid_claim_with_bad_claim_amount() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let mut deps = mock_dependencies();

        let claim_coins = vec![
            // notice these are not alphabetically sorted
            Coin {
                denom: "uxyz".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(8u128), // trying to claim too much of uosmo
            },
        ];

        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();

        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![
                    Coin {
                        denom: "uosmo".to_string(),
                        amount: Uint128::from(7u128),
                    },
                    Coin {
                        denom: "uxyz".to_string(),
                        amount: Uint128::from(1u128),
                    },
                ]),
            )
            .unwrap();

        let result = super::is_valid_claim(
            deps.as_ref(),
            Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            USER_MERKLE_PROOF.to_string(),
        );
        if let Err(ContractError::FailedVerifyProof {}) = result {
            assert!(true); // expected
        } else {
            panic!("unexpected result");
        }
    }
}
