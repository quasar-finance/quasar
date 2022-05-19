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
	opWeightMsgRegisterAccount = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRegisterAccount int = 100

	opWeightMsgCreatePool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePool int = 100

	opWeightMsgJoinPool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgJoinPool int = 100

	opWeightMsgExitPool = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgExitPool int = 100

	opWeightMsgIbcTransfer = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgIbcTransfer int = 100

	opWeightMsgForwardIbcTransfer = "op_weight_msg_forward_ibc_transfer"
	// TODO: Determine the simulation weight value
	defaultWeightMsgForwardIbcTransfer int = 100

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

	var weightMsgRegisterAccount int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRegisterAccount, &weightMsgRegisterAccount, nil,
		func(_ *rand.Rand) {
			weightMsgRegisterAccount = defaultWeightMsgRegisterAccount
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRegisterAccount,
		intergammsimulation.SimulateMsgRegisterAccount(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgCreatePool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCreatePool, &weightMsgCreatePool, nil,
		func(_ *rand.Rand) {
			weightMsgCreatePool = defaultWeightMsgCreatePool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCreatePool,
		intergammsimulation.SimulateMsgCreatePool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgJoinPool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgJoinPool, &weightMsgJoinPool, nil,
		func(_ *rand.Rand) {
			weightMsgJoinPool = defaultWeightMsgJoinPool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgJoinPool,
		intergammsimulation.SimulateMsgJoinPool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgExitPool int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgExitPool, &weightMsgExitPool, nil,
		func(_ *rand.Rand) {
			weightMsgExitPool = defaultWeightMsgExitPool
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgExitPool,
		intergammsimulation.SimulateMsgExitPool(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgIbcTransfer int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgIbcTransfer, &weightMsgIbcTransfer, nil,
		func(_ *rand.Rand) {
			weightMsgIbcTransfer = defaultWeightMsgIbcTransfer
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgIbcTransfer,
		intergammsimulation.SimulateMsgIbcTransfer(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgForwardIbcTransfer int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgForwardIbcTransfer, &weightMsgForwardIbcTransfer, nil,
		func(_ *rand.Rand) {
			weightMsgForwardIbcTransfer = defaultWeightMsgForwardIbcTransfer
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgForwardIbcTransfer,
		intergammsimulation.SimulateMsgForwardIbcTransfer(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
