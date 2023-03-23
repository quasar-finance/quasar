use cosmwasm_std::{Storage, Addr};

use crate::{error::ContractError, state::ADMIN};


// check if sender is the admin, if no admin is set, set sender as the admin
pub fn check_or_set_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.may_load(storage)?;
    if let Some(admin) = admin {
        Ok(&admin == sender)
    } else {
        ADMIN.save(storage, &sender)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use super::*;

    #[test]
    fn test_admin() {
        let mut deps = mock_dependencies();
        let sender1 = Addr::unchecked("alice");
        let sender2 = Addr::unchecked("eve");

        assert!(ADMIN.may_load(deps.as_mut().storage).unwrap().is_none());
        assert!(check_or_set_admin(deps.as_mut().storage, &sender1).unwrap());
        assert_eq!(&ADMIN.load(deps.as_mut().storage).unwrap(), &sender1);
        assert_eq!(check_or_set_admin(deps.as_mut().storage, &sender1), Ok(true));
        assert_eq!(check_or_set_admin(deps.as_mut().storage, &sender2), Ok(false));

    }
}