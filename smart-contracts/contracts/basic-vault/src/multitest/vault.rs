use crate::multitest::common::*;
use crate::multitest::suite::*;

#[test]
fn bond_and_receive_callback() {
    let mut suite = QuasarVaultSuite::init(None, None).unwrap();
    let err = suite
        .bond(
            &suite.user.clone(),
            vec![Coin {
                denom: LOCAL_DENOM.to_string(),
                amount: Uint128::from(1000u128),
            }],
        )
        .unwrap_err();
    assert_eq!(err,VaultContractError::Std(cosmwasm_std::StdError::GenericErr{ msg: "we failed here ser 1 Uint128(333) [Coin { denom: \"ibc/ilovemymom\", amount: Uint128(333) }]".to_string() }));
}
