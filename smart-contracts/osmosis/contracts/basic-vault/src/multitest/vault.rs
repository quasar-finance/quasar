use crate::multitest::common::*;
use crate::multitest::suite::*;

#[test]
fn try_bond() {
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
    // this error happens because our ibc channel is not open yet
    assert_eq!(
        err,
        VaultContractError::Std(cosmwasm_std::StdError::GenericErr {
            msg: "type: alloc::string::String; key: [69, 63, 71, 5F, 63, 68, 61, 6E, 6E, 65, 6C] not found".to_string()
        })
    );
}
