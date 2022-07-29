package keeper

import (
	"github.com/quasarlabs/quasarnode/x/qbank/types"
)

var _ types.QueryServer = Keeper{}
