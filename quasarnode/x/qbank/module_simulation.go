package qbank

import (
	"math/rand"

	"github.com/abag/quasarnode/testutil/sample"
	qbanksimulation "github.com/abag/quasarnode/x/qbank/simulation"
	"github.com/abag/quasarnode/x/qbank/types"
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
	_ = qbanksimulation.FindAccount
	_ = simappparams.StakePerAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
)

const (
	opWeightMsgRequestDeposit = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRequestDeposit int = 100

	opWeightMsgRequestWithdraw = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRequestWithdraw int = 100

	// this line is used by starport scaffolding # simapp/module/const
)

// GenerateGenesisState creates a randomized GenState of the module
func (AppModule) GenerateGenesisState(simState *module.SimulationState) {
	accs := make([]string, len(simState.Accounts))
	for i, acc := range simState.Accounts {
		accs[i] = acc.Address.String()
	}
	qbankGenesis := types.GenesisState{
		// this line is used by starport scaffolding # simapp/module/genesisState
	}
	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(&qbankGenesis)
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

	var weightMsgRequestDeposit int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRequestDeposit, &weightMsgRequestDeposit, nil,
		func(_ *rand.Rand) {
			weightMsgRequestDeposit = defaultWeightMsgRequestDeposit
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRequestDeposit,
		qbanksimulation.SimulateMsgRequestDeposit(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgRequestWithdraw int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRequestWithdraw, &weightMsgRequestWithdraw, nil,
		func(_ *rand.Rand) {
			weightMsgRequestWithdraw = defaultWeightMsgRequestWithdraw
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRequestWithdraw,
		qbanksimulation.SimulateMsgRequestWithdraw(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
