package types

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

type DepositHooks interface {
	OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin)
}

var _ DepositHooks = MultiDepositHooks{}

type MultiDepositHooks []DepositHooks

func NewMultiDepositHooks(hooks ...DepositHooks) MultiDepositHooks {
	return hooks
}

func (h MultiDepositHooks) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	fmt.Printf("\n MultiDepositHooks : OnDeposit \n")
	for i := range h {
		fmt.Printf("\n MultiDepositHooks : OnDeposit Index = %d\n", i)
		h[i].OnDeposit(ctx, vaultID, coin)
	}
}
