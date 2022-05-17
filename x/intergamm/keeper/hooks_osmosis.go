package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types/osmosis"
)

func (k *Keeper) AddHook_Osmosis_MsgCreateBalancerPool(ih osmosis.Hooks_MsgCreateBalancerPool) {
	k.hooks_Osmosis_MsgCreateBalancerPool = append(k.hooks_Osmosis_MsgCreateBalancerPool, ih)
}
