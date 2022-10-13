package qoracle

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	qoraclesimulation "github.com/quasarlabs/quasarnode/x/qoracle/simulation"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// avoid unused import issue
var (
	_ = sample.AccAddressStr
	_ = qoraclesimulation.FindAccount
	_ = simappparams.StakePerAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
)

const (
	opWeightMsgUpdateOsmosisChainParams = "op_weight_msg_update_osmosis_params"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdateOsmosisChainParams int = 100

	// this line is used by starport scaffolding # simapp/module/const
)

// GenerateGenesisState creates a randomized GenState of the module
func (AppModule) GenerateGenesisState(simState *module.SimulationState) {
	accs := make([]string, len(simState.Accounts))
	for i, acc := range simState.Accounts {
		accs[i] = acc.Address.String()
	}
	qoracleGenesis := types.GenesisState{
		// this line is used by starport scaffolding # simapp/module/genesisState
	}
	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(&qoracleGenesis)
}

// ProposalContents doesn't return any content functions for governance proposals
func (AppModule) ProposalContents(_ module.SimulationState) []simtypes.WeightedProposalContent {
	return nil
}

// RandomizedParams creates randomized  param changes for the simulator
func (am AppModule) RandomizedParams(_ *rand.Rand) []simtypes.ParamChange {
	return []simtypes.ParamChange{}
}

// RegisterStoreDecoder registers a decoder
func (am AppModule) RegisterStoreDecoder(_ sdk.StoreDecoderRegistry) {}

// WeightedOperations returns the all the gov module operations with their respective weights.
func (am AppModule) WeightedOperations(simState module.SimulationState) []simtypes.WeightedOperation {
	operations := make([]simtypes.WeightedOperation, 0)

	var weightMsgUpdateOsmosisChainParams int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgUpdateOsmosisChainParams, &weightMsgUpdateOsmosisChainParams, nil,
		func(_ *rand.Rand) {
			weightMsgUpdateOsmosisChainParams = defaultWeightMsgUpdateOsmosisChainParams
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgUpdateOsmosisChainParams,
		qoraclesimulation.SimulateMsgUpdateOsmosisChainParams(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
