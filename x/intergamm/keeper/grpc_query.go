package keeper

import (
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

var _ types.QueryServer = Keeper{}
