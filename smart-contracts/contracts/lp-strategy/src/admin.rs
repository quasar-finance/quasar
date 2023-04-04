use cosmwasm_std::{Addr, Storage};

use crate::{
    error::ContractError,
    state::{DEPOSITOR},
};

// check if sender is the admin
pub fn check_depositor(storage: &mut dyn Storage, sender: &Addr) -> Result<bool, ContractError> {
    let depositor = DEPOSITOR.load(storage)?;
    Ok(&depositor == sender)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_admin() {
        let mut deps = mock_dependencies();
        let sender1 = Addr::unchecked("alice");
        let sender2 = Addr::unchecked("eve");
        
        DEPOSITOR.save(deps.as_mut().storage, &sender1).unwrap();
        assert!(check_depositor(deps.as_mut().storage, &sender1).unwrap());
        assert_eq!(check_depositor(deps.as_mut().storage, &sender1), Ok(true));
        assert_eq!(check_depositor(deps.as_mut().storage, &sender2), Ok(false));
    }
}
