use crate::{proto::CosmosResponse, state::QUERY_RESULT_COUNTER, ContractError};
use cosmwasm_std::{attr, DepsMut, Env, IbcBasicResponse, IbcPacket};

// for our sample origin callback, we increment the query counter and leave it at that
pub fn handle_sample_callback(
    deps: DepsMut,
    _env: Env,
    response: CosmosResponse,
    _original: IbcPacket,
) -> Result<IbcBasicResponse, ContractError> {
    let attrs = vec![
        attr("action", "acknowledge"),
        attr("num_messages", response.responses.len().to_string()),
        attr("success", "true"),
    ];

    // Store result counter.
    let mut counter = QUERY_RESULT_COUNTER.load(deps.storage)?;
    counter += response.responses.len() as u64;
    QUERY_RESULT_COUNTER.save(deps.storage, &counter)?;
    Ok(IbcBasicResponse::new().add_attributes(attrs))
}
