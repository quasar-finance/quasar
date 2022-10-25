use cosmwasm_std::{
    DepsMut, Order, OverflowError, OverflowOperation, StdError, StdResult, Storage,
};
use cw_storage_plus::Map;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::VecDeque;

use crate::state::{WithdrawRequest, WITHDRAW_QUEUE};

pub fn enqueue<'a, T>(deps: DepsMut, value: T, queue: Map<u128, T>) -> StdResult<()>
where
    T: Serialize + DeserializeOwned + Default,
{
    // find the last element in the queue and extract key
    let q: VecDeque<_> = queue
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<_>>()
        .unwrap();
    // TODO remove this awful bit once we refactor queues
    let default = &(0, T::default());
    let (last, _) = q.back().unwrap_or(default).clone();
    let next = last.checked_add(1);
    if next.is_none() {
        return Err(StdError::overflow(OverflowError {
            operation: OverflowOperation::Add,
            operand1: last.to_string(),
            operand2: "1".to_string(),
        }));
    }
    queue.save(deps.storage, next.unwrap(), &value)
}

pub fn dequeue<'a, T>(deps: DepsMut, queue: Map<u128, T>) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    // find the first element in the queue and extract value
    let mut q: VecDeque<_> = queue
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<_>>()
        .unwrap();
    match q.pop_front() {
        None => None,
        Some((key, value)) => {
            // remove the key from the map
            queue.remove(deps.storage, key);
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
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Uint128;

    #[test]
    fn enqueue_dequeue_one_works() {
        let mut deps = mock_dependencies();
        let queue: Map<u128, WithdrawRequest> = Map::new("queue");
        let req = WithdrawRequest {
            denom: "uqsar".to_string(),
            amount: Uint128::new(100_000),
            owner: "alice".to_string(),
        };
        enqueue::<WithdrawRequest>(deps.as_mut(), req.clone(), queue.clone()).unwrap();
        let res = dequeue::<WithdrawRequest>(deps.as_mut(), queue).unwrap();
        assert_eq!(req, res)
    }

    #[test]
    fn enqueue_dequeue_multiple_works() {
        let mut deps = mock_dependencies();
        let queue: Map<u128, WithdrawRequest> = Map::new("queue");
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
        enqueue::<WithdrawRequest>(deps.as_mut(), req1.clone(), queue.clone()).unwrap();
        enqueue::<WithdrawRequest>(deps.as_mut(), req2.clone(), queue.clone()).unwrap();
        let res1 = dequeue::<WithdrawRequest>(deps.as_mut(), queue.clone()).unwrap();
        let res2 = dequeue::<WithdrawRequest>(deps.as_mut(), queue).unwrap();
        assert_eq!(req1, res1);
        assert_eq!(req2, res2)
    }
}
