package keeper

import (
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

var _ types.QueryServer = Keeper{}
