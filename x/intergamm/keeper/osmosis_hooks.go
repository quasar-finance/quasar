package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

type OsmosisHooks struct {
	hooksMsgCreateBalancerPool []func(sdk.Context, types.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])
}

func (oh *OsmosisHooks) AddHookMsgCreateBalancerPool(h func(sdk.Context, types.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])) {
	oh.hooksMsgCreateBalancerPool = append(oh.hooksMsgCreateBalancerPool, h)
}
