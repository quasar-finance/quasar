package types

import sdk "github.com/cosmos/cosmos-sdk/types"

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
type EpochUserDenomWeight struct {
	UserAcc string
	Denom   string
	Weight  sdk.Dec
}
