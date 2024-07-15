use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    // create the initial position while instantiating the contract
    InstantiateCreatePosition = 1,
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
    // create a new position from free funds
    RangeNewCreatePosition,
    // Add funds to a position
    RangeAddToPosition,
    // swap
    Swap,
    /// Merge positions, used to merge positions
    Merge,

    // handle user withdraws after liquidity is removed from the main position
    WithdrawUserMain,
    // handle user withdraws after user funds are withdrawn from all positions and free balance pro rato
    WithdrawUserProRato,
    // after creating a denom in initialization, register the created denom
    CreateDenom,
    /// to merge positions, we need to withdraw positions, used internally for merging
    WithdrawMerge,
    // create a new singular position in the merge, used internally for merging
    CreatePositionMerge,
    /// reply for final swap success in auto compound
    Autocompound,
    /// handle exact deposit swap reply
    AnyDepositSwap,
    #[default]
    Unknown,
}
