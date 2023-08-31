use num_enum::{FromPrimitive, IntoPrimitive};

use cosmwasm_std::{coin, Decimal, DepsMut, Env, Response, SubMsgResult};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenomResponse, MsgMint};

use crate::ContractError;
use crate::state::{Position, POSITION, VAULT_DENOM};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    // handles position creation for a user deposit
    DepositCreatePosition = 1,
    // create the initial position while instantiating the contract
    InstantiateCreatePosition,
    // when handling rewards, we first collect incentives, then collect rewards
    CollectIncentives,
    // after gathering rewards, we divide them over share holders
    CollectSpreadRewards,

    // withdraw position
    WithdrawPosition,
    // create position in the modify range inital step
    RangeInitialCreatePosition,
    // create position in the modify range iteration step
    RangeIterationCreatePosition,
    // swap
    Swap,
    /// Merge positions, used to merge positions
    Merge,

    // handle user withdraws after liquidity is removed from the position
    WithdrawUser,
    // after creating a denom in initialization, register the created denom
    CreateDenom,
    /// to merge positions, we need to withdraw positions, used internally for merging
    WithdrawMerge,
    // create a new singular position in the merge, used internally for merging
    CreatePositionMerge,
    #[default]
    Unknown,
}

pub fn handle_create_denom_reply(
    deps: DepsMut,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreateDenomResponse = data.try_into()?;
    VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

    Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
}

pub fn handle_instantiate_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = data.try_into()?;
    POSITION.save(
        deps.storage,
        &Position {
            position_id: response.position_id,
        },
    )?;

    let liquidity = Decimal::raw(response.liquidity_created.parse()?); // DOUBTS: is this shares amount?
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // todo do we want to mint the initial mint to the instantiater, or just not care?
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(liquidity.atomics().u128(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("initial_position", response.position_id.to_string())
        .add_attribute("initial_liquidity", response.liquidity_created))
}
