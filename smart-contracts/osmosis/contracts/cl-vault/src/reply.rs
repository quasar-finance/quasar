use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    InstantiateCreatePosition = 1,
    CollectIncentives,
    CollectSpreadRewards,
    WithdrawPosition,
    Swap,
    Merge,
    WithdrawUser,
    CreateDenom,
    WithdrawMerge,
    CreatePositionMerge,
    Autocompound,
    AnyDepositSwap,
    #[default]
    Unknown,
    RangeCreatePosition,
}
