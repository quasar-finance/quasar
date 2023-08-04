pub enum Replies {
    //
    DepositCreatePool = 1,
    // when handling rewards, we first collect incentives, then collect rewards
    CollectIncentives,
    // after gathering rewards, we divide them over share holders
    CollectSpreadRewards,
    // create position
    CreatePosition,
    Unknown,
}
