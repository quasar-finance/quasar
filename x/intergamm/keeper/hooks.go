package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
)

func (k *Keeper) AddHookOsmosisMsgCreateBalancerPool(ih types.HooksOsmosisMsgCreateBalancerPool) {
	k.hooksOsmosisMsgCreateBalancerPool = append(k.hooksOsmosisMsgCreateBalancerPool, ih)
}
