use num_enum::{FromPrimitive, IntoPrimitive};

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
    //withdraw position before migration of pool
    WithdrawToMigrate,
    //create position after migration
    CreateMigratedPosition,
    #[default]
    Unknown,
}
