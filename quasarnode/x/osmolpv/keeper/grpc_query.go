package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
)

var _ types.QueryServer = Keeper{}
