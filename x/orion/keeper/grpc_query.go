package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
)

var _ types.QueryServer = Keeper{}
