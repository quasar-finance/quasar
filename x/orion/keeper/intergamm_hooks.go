package keeper

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) OnIcaAcknowledgement(ctx sdk.Context) {
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("HOOK CALLED")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
}
