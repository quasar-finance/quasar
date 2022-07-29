package keeper

import (
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

var _ types.QueryServer = Keeper{}
