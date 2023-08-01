use apollo_cw_asset::{Asset, AssetInfo};
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use cw_dex_router::helpers::receive_asset;

use crate::{state::BASE_TOKEN, ContractError};

pub(crate) fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    amount: Uint128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Unwrap recipient or use caller's address
    let _recipient = recipient.map_or(Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

    // Receive the assets to the contract
    let _receive_res = receive_asset(
        info,
        &env,
        &Asset::new(BASE_TOKEN.load(deps.storage)?, amount),
    )?;

    // TODO we should accept two tokens of a position
    // Check that only the expected amount of base token was sent
    if info.funds.len() > 1 {
        return Err(ContractError::UnexpectedFunds {
            expected: vec![Coin {
                denom: BASE_TOKEN.load(deps.storage)?.to_string(),
                amount,
            }],
            actual: info.funds.clone(),
        });
    }

    // If base token is a native token it was sent in the `info.funds` and is
    // already part of the contract balance. That is not the case for a cw20 token,
    // which will be received when the above `receive_res` is handled.
    let _user_deposit_amount = match BASE_TOKEN.load(deps.storage)? {
        AssetInfo::Cw20(_) => Uint128::zero(),
        AssetInfo::Native(_) => amount,
    };

    todo!()

    // // Compound. Also stakes the users deposit
    // let compound_res = self.compound(deps, &env, user_deposit_amount)?;

    // // Mint vault tokens to recipient
    // let mint_res = Response::new().add_message(
    //     CallbackMsg::MintVaultToken {
    //         amount,
    //         recipient: recipient.clone(),
    //     }
    //     .into_cosmos_msg(&env)?,
    // );

    // let event = Event::new("apollo/vaults/execute_staking").add_attributes(vec![
    //     attr("action", "deposit"),
    //     attr("recipient", recipient),
    //     attr("amount", amount),
    // ]);

    // // Merge responses and add message to mint vault token
    // Ok(merge_responses(vec![receive_res, compound_res, mint_res]).add_event(event))
}
