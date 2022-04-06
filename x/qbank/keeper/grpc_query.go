package keeper

import (
	"github.com/abag/quasarnode/x/qbank/types"
)

var _ types.QueryServer = Keeper{}
