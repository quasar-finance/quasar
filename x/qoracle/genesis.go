package qoracle

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	qbandkeeper "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/keeper"
	genesistypes "github.com/quasarlabs/quasarnode/x/qoracle/genesis/types"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
)

func InitGenesis(
	ctx sdk.Context,
	qKeeper qoraclekeeper.Keeper,
	bandKeeper qbandkeeper.Keeper,
	osmoKeeper qosmokeeper.Keeper,
	state genesistypes.GenesisState,
) {
	qKeeper.SetParams(ctx, state.Params)

	for _, mapping := range state.DenomSymbolMappings {
		qKeeper.SetDenomSymbolMapping(ctx, mapping)
	}

	qbandkeeper.InitGenesis(ctx, bandKeeper, state.BandchainGenesisState)
	qosmokeeper.InitGenesis(ctx, osmoKeeper, state.OsmosisGenesisState)
}

func ExportGenesis(
	ctx sdk.Context,
	qKeeper qoraclekeeper.Keeper,
	bandKeeper qbandkeeper.Keeper,
	osmoKeeper qosmokeeper.Keeper,
) *genesistypes.GenesisState {
	return genesistypes.NewGenesisState(
		qKeeper.GetParams(ctx),
		qbandkeeper.ExportGenesis(ctx, bandKeeper),
		qosmokeeper.ExportGenesis(ctx, osmoKeeper),
	)
}
