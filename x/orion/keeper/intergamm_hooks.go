package keeper

import (
	"fmt"

	intergammtypesosmosis "github.com/abag/quasarnode/x/intergamm/types/osmosis"
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

func (k Keeper) Handle_MsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypesosmosis.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) {
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("HOOK CALLED")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
	fmt.Println(ex.HasError())
	fmt.Println(ex.Error)
	fmt.Println("")
	fmt.Println(ex.Response)
}
