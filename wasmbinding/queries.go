package wasmbinding

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

type QueryPlugin struct {
	intergammKeeper *intergammkeeper.Keeper
	qoracleKeeper   *qoraclekeeper.Keeper
}

// NewQueryPlugin returns a reference to a new QueryPlugin.
func NewQueryPlugin(gk *intergammkeeper.Keeper) *QueryPlugin {
	return &QueryPlugin{
		intergammKeeper: gk,
	}
}

func (qp QueryPlugin) GetPoolInfo(ctx sdk.Context, poolID string) (types.PoolInfo, error) {
	pool, found := qp.qoracleKeeper.GetPoolInfo(ctx, poolID)
	if !found {
		return nil, fmt.Errorf("failed to find pool for poolID: %s", poolID)
	}

	return pool, nil
}
