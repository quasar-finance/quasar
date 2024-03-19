use crate::{
    error::ContractError,
    state::{CLAIMED_INCENTIVES, MERKLE_ROOT},
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use cosmwasm_std::{Addr, Deps};
use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleProof};

use super::CoinVec;

pub fn is_valid_claim(
    deps: Deps,
    address: &Addr,
    coins: &CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
) -> Result<CoinVec, ContractError> {
    let merkle_root = MERKLE_ROOT.load(deps.storage)?;

    // the format of this will look like "addr1000utokena2000utokenb"
    let claim_string = format!("{}{}", address.as_str(), coins);

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

    if claimed_amount.ge(coins) {
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
    let root_hash = STANDARD.decode(merkle_root).unwrap();
    let to_verify_hash = Sha256::hash(to_verify.as_bytes());

    let proof = MerkleProof::<Sha256>::new(proof_hashes);

    if !proof.verify(
        root_hash.try_into().unwrap(),
        leaf_indices,
        &[to_verify_hash],
        total_leaves_count,
    ) {
        return Err(ContractError::FailedVerifyProof {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use cosmwasm_std::{testing::mock_dependencies, Addr, Coin, Uint128};

    use crate::{
        incentives::{
            helpers::{is_valid_claim, verify_proof},
            CoinVec,
        },
        state::{CLAIMED_INCENTIVES, MERKLE_ROOT},
        ContractError,
    };

    // Test constants
    const MERKLE_ROOT_STRING: &str = "0hGvbH+l9pdPgOmJY6wZuwjsrvtPsuslgTURavrUP6I=";
    const MERKLE_ROOT_INVALID_STRING: &str = "INVALIDROOTRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI=";

    const USER_ADDRESS: &str = "osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt";

    const CLAIM_PROOF_STRING: &str = "osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt900000000ugauge";
    const CLAIM_PROOF_INVALID_STRING: &str =
        "osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt999999999ugauge";

    const CLAIM_AMOUNT: u128 = 900000000;
    const CLAIM_AMOUNT_INVALID: u128 = 999999999;

    // Utils functions
    fn get_proof_hashes() -> Vec<[u8; 32]> {
        let proof = vec![
            STANDARD
                .decode("R6J/QIhrqN4KxMa7ZhaCm/6J7ibT7HHcw9KKRV4ML0k=")
                .unwrap(),
            STANDARD
                .decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=")
                .unwrap(),
            STANDARD
                .decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=")
                .unwrap(),
            STANDARD
                .decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=")
                .unwrap(),
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
        proof_hashes
    }

    /// IS VALID CLAIM
    #[test]
    fn test_is_valid_claim_true() {
        // this test is taken directly from the testdata. See the README.md of this contract
        let mut deps = mock_dependencies();

        let claim_coins = vec![Coin {
            denom: "ugauge".to_string(),
            amount: Uint128::from(CLAIM_AMOUNT),
        }];

        // Contract state write
        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();

        // Is valid claim
        let result = is_valid_claim(
            deps.as_ref(),
            &Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            get_proof_hashes(),
            0usize,
            10usize,
        );

        assert_eq!(result.unwrap(), claim_coins.into());
    }

    #[test]
    fn test_is_valid_claim_some_already_claimed() {
        let mut deps = mock_dependencies();

        let claim_coins = vec![Coin {
            denom: "ugauge".to_string(),
            amount: Uint128::from(CLAIM_AMOUNT),
        }];

        // Contract state write to simulate already claimed amount
        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();
        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![Coin {
                    denom: "ugauge".to_string(),
                    amount: Uint128::from(1000u128),
                }]),
            )
            .unwrap();

        // Is valid claim
        let result = is_valid_claim(
            deps.as_ref(),
            &Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            get_proof_hashes(),
            0usize,
            10usize,
        );

        // Assert
        let expected_claim = vec![Coin {
            denom: "ugauge".to_string(),
            amount: Uint128::from(CLAIM_AMOUNT.checked_sub(1000u128).unwrap()),
        }];
        assert_eq!(result.unwrap(), expected_claim.into());
    }

    #[test]
    fn test_is_valid_claim_all_already_claimed() {
        let mut deps = mock_dependencies();

        let claim_coins = vec![Coin {
            denom: "ugauge".to_string(),
            amount: Uint128::from(CLAIM_AMOUNT),
        }];

        // Contract state write
        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();
        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![Coin {
                    denom: "ugauge".to_string(),
                    amount: Uint128::from(CLAIM_AMOUNT),
                }]),
            )
            .unwrap();

        // Is valid claim
        let result = is_valid_claim(
            deps.as_ref(),
            &Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            get_proof_hashes(),
            0usize,
            10usize,
        );

        assert_eq!(
            result.unwrap_err(),
            ContractError::IncentivesAlreadyClaimed {}
        );
    }

    #[test]
    fn test_is_valid_claim_with_bad_claim_amount() {
        let mut deps = mock_dependencies();

        let claim_coins = vec![Coin {
            denom: "ugauge".to_string(),
            amount: Uint128::from(CLAIM_AMOUNT_INVALID),
        }];

        // Contract state write
        MERKLE_ROOT
            .save(deps.as_mut().storage, &MERKLE_ROOT_STRING.to_string())
            .unwrap();
        CLAIMED_INCENTIVES
            .save(
                deps.as_mut().storage,
                Addr::unchecked(USER_ADDRESS),
                &CoinVec(vec![Coin {
                    denom: "ugauge".to_string(),
                    amount: Uint128::from(1u128),
                }]),
            )
            .unwrap();

        // Is valid claim
        let result = is_valid_claim(
            deps.as_ref(),
            &Addr::unchecked(USER_ADDRESS),
            &claim_coins.clone().into(),
            get_proof_hashes(),
            0usize,
            10usize,
        );

        assert_eq!(result.unwrap_err(), ContractError::FailedVerifyProof {})
    }

    /// VERIFY PROOF
    #[test]
    fn test_verify_success() {
        verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            get_proof_hashes(),
            &[0usize],
            10usize,
            CLAIM_PROOF_STRING,
        )
        .unwrap();
    }

    #[test]
    fn test_verify_bad_root() {
        let result = verify_proof(
            &MERKLE_ROOT_INVALID_STRING.to_string(),
            get_proof_hashes(),
            &[0usize],
            10usize,
            CLAIM_PROOF_STRING,
        );

        assert_eq!(result.unwrap_err(), ContractError::FailedVerifyProof {})
    }

    #[test]
    fn test_verify_bad_claim() {
        let result = verify_proof(
            &MERKLE_ROOT_STRING.to_string(),
            get_proof_hashes(),
            &[0usize],
            10usize,
            CLAIM_PROOF_INVALID_STRING,
        );

        assert_eq!(result.unwrap_err(), ContractError::FailedVerifyProof {})
    }
}
