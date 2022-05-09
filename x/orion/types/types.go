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
	Weight  sdk.Dec
	Coin    sdk.Coin
}

// EpochDenomWeight is used to create a pair of denom and its weight on a particular epoch day
type EpochDenomWeight struct {
	Denom  string
	Weight sdk.Dec
}

// EpochUsersReward is used to pair user account and associated reward coins on a particular epoch day
type EpochUsersReward struct {
	UserAcc string
	Rewards sdk.Coins
}

// EpochDenomReward is used to create a pair of denom and its associated rewards
type EpochDenomReward struct {
	Denom   string
	Rewards sdk.Coins
}

// DepositDayLockupPair is used to create pairs of deposit day and lockup period done on that day.
type DepositDayLockupPair struct {
	EpochDay     uint64
	LockupPeriod qbanktypes.LockupTypes
}

// UserDenomInfo is used to hold denom level information for a particular user
type UserDenomInfo struct {
	Denom  string
	Weight sdk.Dec
	Amt    sdk.Int
	Reward sdk.Coins
}

// UserInfo is used to hold denom level info and total rewards for a particular user
type UserInfo struct {
	UserAcc     string
	DenomMap    map[string]UserDenomInfo
	TotalReward sdk.Coins
}

// UserInfoMap is a map of user account to UserInfo
type UserInfoMap map[string]UserInfo
