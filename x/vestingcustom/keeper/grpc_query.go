package keeper

import (
	"github.com/quasarlabs/quasarnode/x/vestingcustom/types"
)

var _ types.QueryServer = Keeper{}
