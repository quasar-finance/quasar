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
	opWeightMsgCreatePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePoolPosition int = 100

	opWeightMsgUpdatePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdatePoolPosition int = 100

	opWeightMsgDeletePoolPosition = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgDeletePoolPosition int = 100

	opWeightMsgCreatePoolRanking = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePoolRanking int = 100

	opWeightMsgUpdatePoolRanking = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdatePoolRanking int = 100

	opWeightMsgDeletePoolRanking = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgDeletePoolRanking int = 100

	opWeightMsgCreatePoolSpotPrice = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePoolSpotPrice int = 100

	opWeightMsgUpdatePoolSpotPrice = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdatePoolSpotPrice int = 100

	opWeightMsgDeletePoolSpotPrice = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgDeletePoolSpotPrice int = 100

	opWeightMsgCreatePoolInfo = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCreatePoolInfo int = 100

	opWeightMsgUpdatePoolInfo = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgUpdatePoolInfo int = 100

	opWeightMsgDeletePoolInfo = "op_weight_msg_create_chain"
	// TODO: Determine the simulation weight value
	defaultWeightMsgDeletePoolInfo int = 100

	opWeightMsgStablePrice = "op_weight_msg_stable_price"
	// TODO: Determine the simulation weight value
	defaultWeightMsgStablePrice int = 100

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
		PoolPositionList: []types.PoolPosition{
			{
				Creator: sample.AccAddressStr(),
				PoolId:  "0",
			},
			{
				Creator: sample.AccAddressStr(),
				PoolId:  "1",
			},
		},
		PoolSpotPriceList: []types.PoolSpotPrice{
			{
				Creator:  sample.AccAddressStr(),
				PoolId:   "0",
				DenomIn:  "0",
				DenomOut: "0",
			},
			{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "1",
				DenomOut: "1",
			},
		},
		PoolInfoList: []types.PoolInfo{
			{
				Creator: sample.AccAddressStr(),
				PoolId:  "0",
			},
			{
				Creator: sample.AccAddressStr(),
				PoolId:  "1",
			},
		},
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

	var weightMsgCreatePoolRanking int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCreatePoolRanking, &weightMsgCreatePoolRanking, nil,
		func(_ *rand.Rand) {
			weightMsgCreatePoolRanking = defaultWeightMsgCreatePoolRanking
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCreatePoolRanking,
		qoraclesimulation.SimulateMsgCreatePoolRanking(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgUpdatePoolRanking int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgUpdatePoolRanking, &weightMsgUpdatePoolRanking, nil,
		func(_ *rand.Rand) {
			weightMsgUpdatePoolRanking = defaultWeightMsgUpdatePoolRanking
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgUpdatePoolRanking,
		qoraclesimulation.SimulateMsgUpdatePoolRanking(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgDeletePoolRanking int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgDeletePoolRanking, &weightMsgDeletePoolRanking, nil,
		func(_ *rand.Rand) {
			weightMsgDeletePoolRanking = defaultWeightMsgDeletePoolRanking
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgDeletePoolRanking,
		qoraclesimulation.SimulateMsgDeletePoolRanking(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgCreatePoolSpotPrice int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCreatePoolSpotPrice, &weightMsgCreatePoolSpotPrice, nil,
		func(_ *rand.Rand) {
			weightMsgCreatePoolSpotPrice = defaultWeightMsgCreatePoolSpotPrice
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCreatePoolSpotPrice,
		qoraclesimulation.SimulateMsgCreatePoolSpotPrice(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgUpdatePoolSpotPrice int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgUpdatePoolSpotPrice, &weightMsgUpdatePoolSpotPrice, nil,
		func(_ *rand.Rand) {
			weightMsgUpdatePoolSpotPrice = defaultWeightMsgUpdatePoolSpotPrice
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgUpdatePoolSpotPrice,
		qoraclesimulation.SimulateMsgUpdatePoolSpotPrice(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgDeletePoolSpotPrice int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgDeletePoolSpotPrice, &weightMsgDeletePoolSpotPrice, nil,
		func(_ *rand.Rand) {
			weightMsgDeletePoolSpotPrice = defaultWeightMsgDeletePoolSpotPrice
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgDeletePoolSpotPrice,
		qoraclesimulation.SimulateMsgDeletePoolSpotPrice(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgCreatePoolInfo int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCreatePoolInfo, &weightMsgCreatePoolInfo, nil,
		func(_ *rand.Rand) {
			weightMsgCreatePoolInfo = defaultWeightMsgCreatePoolInfo
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCreatePoolInfo,
		qoraclesimulation.SimulateMsgCreatePoolInfo(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgUpdatePoolInfo int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgUpdatePoolInfo, &weightMsgUpdatePoolInfo, nil,
		func(_ *rand.Rand) {
			weightMsgUpdatePoolInfo = defaultWeightMsgUpdatePoolInfo
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgUpdatePoolInfo,
		qoraclesimulation.SimulateMsgUpdatePoolInfo(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgDeletePoolInfo int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgDeletePoolInfo, &weightMsgDeletePoolInfo, nil,
		func(_ *rand.Rand) {
			weightMsgDeletePoolInfo = defaultWeightMsgDeletePoolInfo
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgDeletePoolInfo,
		qoraclesimulation.SimulateMsgDeletePoolInfo(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgStablePrice int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgStablePrice, &weightMsgStablePrice, nil,
		func(_ *rand.Rand) {
			weightMsgStablePrice = defaultWeightMsgStablePrice
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgStablePrice,
		qoraclesimulation.SimulateMsgStablePrice(am.accountKeeper, am.bankKeeper, am.keeper),
	))

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
