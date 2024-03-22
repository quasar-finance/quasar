use cosmwasm_std::Coin;
use cw_storage_plus::Item;

pub const AMOUNT_BURNT: Item<Vec<Coin>> = Item::new("burnt_coins");

#[cfg(test)]
mod tests {
    use crate::state::AMOUNT_BURNT;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Coin;

    #[test]
    fn test_burnt_amount() {
        let mut deps = mock_dependencies();

        // Check initial empty amount
        assert_eq!(None, AMOUNT_BURNT.may_load(deps.as_mut().storage).unwrap());

        // Update amount and check
        let coins = vec![
            Coin::new(1_000_000, "denom1"),
            Coin::new(10_000_000, "denom2"),
        ];
        AMOUNT_BURNT.save(deps.as_mut().storage, &coins).unwrap();
        assert_eq!(
            Some(coins.clone()),
            AMOUNT_BURNT.may_load(deps.as_mut().storage).unwrap()
        );
    }
}
