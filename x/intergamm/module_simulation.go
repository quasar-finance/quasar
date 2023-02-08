package intergamm

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	intergammsimulation "github.com/quasarlabs/quasarnode/x/intergamm/simulation"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

// avoid unused import issue
var (
	_ = sample.AccAddress
	_ = simappparams.StakePerAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
)

const (
	opWeightMsgSendToken = "op_weight_msg_send_token"
	// TODO: Determine the simulation weight value
	defaultWeightMsgSendToken int = 100

	opWeightMsgTransmitICATransfer = "op_weight_msg_transmit_ica_transfer"
	// TODO: Determine the simulation weight value
	defaultWeightMsgTransmitICATransfer int = 100

	opWeightMsgRegisterICAOnZone = "op_weight_msg_register_ica_on_zone"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRegisterICAOnZone int = 100

	opWeightMsgRegisterICAOnDenomNativeZone = "op_weight_msg_register_ica_on_denom_native_zone"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRegisterICAOnDenomNativeZone int = 100

	opWeightMsgSendTokenToICA = "op_weight_msg_send_token_to_ica"
	// TODO: Determine the simulation weight value
	defaultWeightMsgSendTokenToICA int = 100

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

	var weightMsgSendToken int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgSendToken, &weightMsgSendToken, nil,
		func(_ *rand.Rand) {
			weightMsgSendToken = defaultWeightMsgSendToken
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgSendToken,
		intergammsimulation.SimulateMsgSendToken(am.accountKeeper, am.bankKeeper, *am.keeper),
	))

	var weightMsgTransmitICATransfer int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgTransmitICATransfer, &weightMsgTransmitICATransfer, nil,
		func(_ *rand.Rand) {
			weightMsgTransmitICATransfer = defaultWeightMsgTransmitICATransfer
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgTransmitICATransfer,
		intergammsimulation.SimulateMsgTransmitICATransfer(am.accountKeeper, am.bankKeeper, *am.keeper),
	))

	var weightMsgRegisterICAOnZone int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRegisterICAOnZone, &weightMsgRegisterICAOnZone, nil,
		func(_ *rand.Rand) {
			weightMsgRegisterICAOnZone = defaultWeightMsgRegisterICAOnZone
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRegisterICAOnZone,
		intergammsimulation.SimulateMsgRegisterICAOnZone(am.accountKeeper, am.bankKeeper, *am.keeper),
	))

	var weightMsgRegisterICAOnDenomNativeZone int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRegisterICAOnDenomNativeZone, &weightMsgRegisterICAOnDenomNativeZone, nil,
		func(_ *rand.Rand) {
			weightMsgRegisterICAOnDenomNativeZone = defaultWeightMsgRegisterICAOnDenomNativeZone
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRegisterICAOnDenomNativeZone,
		intergammsimulation.SimulateMsgRegisterICAOnDenomNativeZone(am.accountKeeper, am.bankKeeper, *am.keeper),
	))

	var weightMsgSendTokenToICA int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgSendTokenToICA, &weightMsgSendTokenToICA, nil,
		func(_ *rand.Rand) {
			weightMsgSendTokenToICA = defaultWeightMsgSendTokenToICA
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgSendTokenToICA,
		intergammsimulation.SimulateMsgSendTokenToICA(am.accountKeeper, am.bankKeeper, *am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
