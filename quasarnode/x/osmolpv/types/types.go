package types

import (
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// UserCoin is used to fetch users deposit of a particular denom on a given
// epoch from the LP positions. This set will be used for the users reward
// calculation based on the percentage.
type UserCoin struct {
	UserAcc string
	Coin    sdk.Coin
	// Weight  sdk.Dec
}

// EpochUserDenomWeight is used to create a users weight on a particular denom
// on given epoch. This set will be used for the users reward calculation based on the percentage.
// Multiple object for the same user can be present in respective data structures
type EpochUserDenomWeight struct {
	UserAcc string
	Denom   string
	Weight  sdk.Dec
}

// EpochDenomWeight is used to create a pair of denom and its weight on a particular epoch day
type EpochDenomWeight struct {
	Denom  string
	Weight sdk.Dec
}

// EpochUserRewards is used to pair user account and associated reward coins on a particular epoch day
type EpochUsersReward struct {
	UserAcc string
	Rewards sdk.Coins
}

type EpochDenomReward struct {
	Denom   string
	Rewards sdk.Coins
}

type DepositDayLockupPair struct {
	Epochday     uint64
	LockupPeriod qbanktypes.LockupTypes
}
