package types

import sdk "github.com/cosmos/cosmos-sdk/types"

type UserCoin struct {
	UserAcc string
	Coin    sdk.Coin
}
