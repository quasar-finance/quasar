use cosmwasm_std::{
    DepsMut, Order, OverflowError, OverflowOperation, StdError, StdResult, Storage,
};
use std::collections::VecDeque;

use crate::state::{WithdrawRequest, WITHDRAW_QUEUE};

pub fn enqueue(deps: DepsMut, value: WithdrawRequest) -> StdResult<()> {
    // find the last element in the queue and extract key
    let queue: VecDeque<_> = WITHDRAW_QUEUE
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<_>>()
        .unwrap();
    let (last, _) = queue
        .back()
        .unwrap_or(&(0, WithdrawRequest::default()))
        .clone();
    let next = last.checked_add(1);
    if next.is_none() {
        return Err(StdError::overflow(OverflowError {
            operation: OverflowOperation::Add,
            operand1: last.to_string(),
            operand2: "1".to_string(),
        }));
    }
    WITHDRAW_QUEUE.save(deps.storage, next.unwrap(), &value)
}

pub fn dequeue(deps: DepsMut) -> Option<WithdrawRequest> {
    handle_dequeue(deps)
}

#[allow(clippy::unnecessary_wraps)]
fn handle_dequeue(deps: DepsMut) -> Option<WithdrawRequest> {
    // find the first element in the queue and extract value
    let mut queue: VecDeque<_> = WITHDRAW_QUEUE
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<_>>()
        .unwrap();
    match queue.pop_front() {
        None => None,
        Some((key, value)) => {
            // remove the key from the map
            WITHDRAW_QUEUE.remove(deps.storage, key);
            // return the underlying value
            Some(value)
        }
    }
}

// pub fn peek(deps: Deps) -> Option<WithdrawRequest> {
//     let mut queue: VecDeque<_> = WITHDRAW_QUEUE.range(deps.storage, None, None, Order::Ascending).collect::<StdResult<_>>().unwrap();
//     // we can use pop front since it doesn't remove the item from the underlying Map
//     let front = queue.pop_front();
//     match front{
//         None => {None}
//         Some((_key, val)) => {
//             return Some(val);
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies};
    use cosmwasm_std::Uint128;

    #[test]
    fn enqueue_dequeue_one_works() {
        let mut deps = mock_dependencies();
        let req = WithdrawRequest {
            denom: "uqsar".to_string(),
            amount: Uint128::new(100_000),
            owner: "alice".to_string(),
        };
        enqueue(deps.as_mut(), req.clone()).unwrap();
        let _mem: Vec<_> = deps.storage.range(None, None, Order::Ascending).collect();
        let res = dequeue(deps.as_mut()).unwrap();
        assert_eq!(req, res)
    }

    #[test]
    fn enqueue_dequeue_multiple_works() {
        let mut deps = mock_dependencies();
        let req1 = WithdrawRequest {
            denom: "uqsar".to_string(),
            amount: Uint128::new(100_000),
            owner: "alice".to_string(),
        };
        let req2 = WithdrawRequest {
            denom: "uqsar".to_string(),
            amount: Uint128::new(100_000),
            owner: "bobbyb".to_string(),
        };
        enqueue(deps.as_mut(), req1.clone()).unwrap();
        enqueue(deps.as_mut(), req2.clone()).unwrap();
        let _mem: Vec<_> = deps.storage.range(None, None, Order::Ascending).collect();
        let res1 = dequeue(deps.as_mut()).unwrap();
        let res2 = dequeue(deps.as_mut()).unwrap();
        assert_eq!(req1, res1);
        assert_eq!(req2, res2)
    }
}
