use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

pub const AMOUNT_BURNT: Map<String, Uint128> = Map::new("burnt_coins");

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;
    #[test]
    fn test_burnt_amount() {
        let mut deps = mock_dependencies();

        // Check initial empty amount
        assert_eq!(
            None,
            AMOUNT_BURNT
                .may_load(deps.as_mut().storage, "any_key".to_string())
                .unwrap()
        );

        AMOUNT_BURNT
            .save(
                deps.as_mut().storage,
                "denom1".to_string(),
                &Uint128::new(1_000_000),
            )
            .unwrap();
        AMOUNT_BURNT
            .save(
                deps.as_mut().storage,
                "denom2".to_string(),
                &Uint128::new(10_000_000),
            )
            .unwrap();
        assert_eq!(
            Uint128::new(1_000_000),
            AMOUNT_BURNT
                .may_load(deps.as_mut().storage, "denom1".to_string())
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            Uint128::new(10_000_000),
            AMOUNT_BURNT
                .may_load(deps.as_mut().storage, "denom2".to_string())
                .unwrap()
                .unwrap()
        );
    }
}
