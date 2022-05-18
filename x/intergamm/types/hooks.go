package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

type HooksOsmosisMsgCreateBalancerPool interface {
	HandleMsgCreateBalancerPool(
		ctx sdk.Context,
		ex Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
	)
}
