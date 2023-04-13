package keeper

import (
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
)

var _ types.QueryServer = Keeper{}
