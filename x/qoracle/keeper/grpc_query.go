package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
)

var _ types.QueryServer = Keeper{}
