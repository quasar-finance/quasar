package qoracle

import (
	"math/rand"

	"github.com/abag/quasarnode/testutil/sample"
	qoraclesimulation "github.com/abag/quasarnode/x/qoracle/simulation"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
)

// avoid unused import issue
var (
	_ = sample.AccAddress
	_ = qoraclesimulation.FindAccount
	_ = simappparams.StakePerAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
)

const (
	opWeightMsgCreatePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePoolPosition int = 100

	opWeightMsgUpdatePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdatePoolPosition int = 100

	opWeightMsgDeletePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgDeletePoolPosition int = 100

	opWeightMsgBalancerPool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgBalancerPool int = 100

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
	qoracleParams := types.DefaultParams()
	return []simtypes.ParamChange{
		simulation.NewSimParamChange(types.ModuleName, string(types.KeyOracleAccounts), func(r *rand.Rand) string {
			return string(types.Amino.MustMarshalJSON(qoracleParams.OracleAccounts))
		}),
	}
}

// RegisterStoreDecoder registers a decoder
func (am AppModule) RegisterStoreDecoder(_ sdk.StoreDecoderRegistry) {}

// WeightedOperations returns the all the gov module operations with their respective weights.
func (am AppModule) WeightedOperations(simState module.SimulationState) []simtypes.WeightedOperation {
	operations := make([]simtypes.WeightedOperation, 0)

	var weightMsgCreatePoolPosition int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCreatePoolPosition, &weightMsgCreatePoolPosition, nil,
		func(_ *rand.Rand) {
			weightMsgCreatePoolPosition = defaultWeightMsgCreatePoolPosition
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCreatePoolPosition,
		qoraclesimulation.SimulateMsgCreatePoolPosition(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgUpdatePoolPosition int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgUpdatePoolPosition, &weightMsgUpdatePoolPosition, nil,
		func(_ *rand.Rand) {
			weightMsgUpdatePoolPosition = defaultWeightMsgUpdatePoolPosition
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgUpdatePoolPosition,
		qoraclesimulation.SimulateMsgUpdatePoolPosition(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgDeletePoolPosition int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgDeletePoolPosition, &weightMsgDeletePoolPosition, nil,
		func(_ *rand.Rand) {
			weightMsgDeletePoolPosition = defaultWeightMsgDeletePoolPosition
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgDeletePoolPosition,
		qoraclesimulation.SimulateMsgDeletePoolPosition(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgBalancerPool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgBalancerPool, &weightMsgBalancerPool, nil,
		func(_ *rand.Rand) {
			weightMsgBalancerPool = defaultWeightMsgBalancerPool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgBalancerPool,
		qoraclesimulation.SimulateMsgBalancerPool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
