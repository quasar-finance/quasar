package intergamm

import (
	"math/rand"

	"github.com/abag/quasarnode/testutil/sample"
	intergammsimulation "github.com/abag/quasarnode/x/intergamm/simulation"
	"github.com/abag/quasarnode/x/intergamm/types"
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
	_ = intergammsimulation.FindAccount
	_ = simappparams.StakePerAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
)

const (
	opWeightMsgSendIbcJoinPool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgSendIbcJoinPool int = 100

	opWeightMsgSendIbcExitPool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgSendIbcExitPool int = 100

	// this line is used by starport scaffolding # simapp/module/const
)

// GenerateGenesisState creates a randomized GenState of the module
func (AppModule) GenerateGenesisState(simState *module.SimulationState) {
	accs := make([]string, len(simState.Accounts))
	for i, acc := range simState.Accounts {
		accs[i] = acc.Address.String()
	}
	intergammGenesis := types.GenesisState{
		// this line is used by starport scaffolding # simapp/module/genesisState
	}
	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(&intergammGenesis)
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

	var weightMsgSendIbcJoinPool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgSendIbcJoinPool, &weightMsgSendIbcJoinPool, nil,
		func(_ *rand.Rand) {
			weightMsgSendIbcJoinPool = defaultWeightMsgSendIbcJoinPool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgSendIbcJoinPool,
		intergammsimulation.SimulateMsgSendIbcJoinPool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgSendIbcExitPool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgSendIbcExitPool, &weightMsgSendIbcExitPool, nil,
		func(_ *rand.Rand) {
			weightMsgSendIbcExitPool = defaultWeightMsgSendIbcExitPool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgSendIbcExitPool,
		intergammsimulation.SimulateMsgSendIbcExitPool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
