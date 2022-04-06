package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
)

var _ types.QueryServer = Keeper{}
