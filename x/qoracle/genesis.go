package qoracle

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	genesistypes "github.com/quasarlabs/quasarnode/x/qoracle/genesis/types"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
)

func InitGenesis(
	ctx sdk.Context,
	qKeeper qoraclekeeper.Keeper,
	osmoKeeper qosmokeeper.Keeper,
	state genesistypes.GenesisState,
) {
	qKeeper.SetParams(ctx, state.Params)
	qosmokeeper.InitGenesis(ctx, osmoKeeper, state.OsmosisGenesisState)
}

func ExportGenesis(
	ctx sdk.Context,
	qKeeper qoraclekeeper.Keeper,
	osmoKeeper qosmokeeper.Keeper,
) *genesistypes.GenesisState {
	return genesistypes.NewGenesisState(
		qKeeper.GetParams(ctx),
		qosmokeeper.ExportGenesis(ctx, osmoKeeper),
	)
}
