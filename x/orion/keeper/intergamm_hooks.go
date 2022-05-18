package keeper

import (
	"fmt"

	intergammtypes "github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

func (k Keeper) HandleMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) {
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("HOOK CALLED")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("Err")
	fmt.Println(ex.HasError())
	fmt.Println(ex.Error)
	fmt.Println("")
	fmt.Println("Req")
	fmt.Println(ex.Request)
	fmt.Println("")
	fmt.Println("Res")
	fmt.Println(ex.Response)
}
