package osmosis

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

type Hooks_MsgCreateBalancerPool interface {
	Handle_MsgCreateBalancerPool(
		ctx sdk.Context,
		ex Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
	)
}
