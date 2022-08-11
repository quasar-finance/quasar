package wasmbinding

import (
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
)

type QueryPlugin struct {
	intergammKeeper *intergammkeeper.Keeper
}

// NewQueryPlugin returns a reference to a new QueryPlugin.
func NewQueryPlugin(gk *intergammkeeper.Keeper) *QueryPlugin {
	return &QueryPlugin{
		intergammKeeper: gk,
	}
}
