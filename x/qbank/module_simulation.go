package qbank

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	banksim "github.com/cosmos/cosmos-sdk/x/bank/simulation"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	qbanksimulation "github.com/quasarlabs/quasarnode/x/qbank/simulation"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
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
	opWeightMsgRequestDeposit = "op_weight_msg_request_deposit"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRequestDeposit int = 100

	opWeightMsgRequestWithdraw = "op_weight_msg_request_withdraw"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRequestWithdraw int = 100

	opWeightMsgClaimRewards = "op_weight_msg_claim_rewards"
	// TODO: Determine the simulation weight value
	defaultWeightMsgClaimRewards int = 100

	opWeightMsgRequestWithdrawAll = "op_weight_msg_request_withdraw_all"
	// TODO: Determine the simulation weight value
	defaultWeightMsgRequestWithdrawAll int = 100

	// this line is used by starport scaffolding # simapp/module/const
)

// GenerateGenesisState creates a randomized GenState of the module
func (AppModule) GenerateGenesisState(simState *module.SimulationState) {
	accs := make([]string, len(simState.Accounts))
	for i, acc := range simState.Accounts {
		accs[i] = acc.Address.String()
	}

	BankGenesisState(simState)

	qbankGenesis := types.GenesisState{
		// this line is used by starport scaffolding # simapp/module/genesisState
	}
	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(&qbankGenesis)
}

// Create genesis state of bank module
func BankGenesisState(simState *module.SimulationState) {
	var sendEnabledParams banktypes.SendEnabledParams
	simState.AppParams.GetOrGenerate(
		simState.Cdc, string(banktypes.KeySendEnabled), &sendEnabledParams, simState.Rand,
		func(r *rand.Rand) { sendEnabledParams = RandomGenesisSendParams(r) },
	)

	var defaultSendEnabledParam bool
	simState.AppParams.GetOrGenerate(
		simState.Cdc, string(banktypes.KeyDefaultSendEnabled), &defaultSendEnabledParam, simState.Rand,
		func(r *rand.Rand) { defaultSendEnabledParam = banksim.RandomGenesisDefaultSendParam(r) },
	)

	numAccs := int64(len(simState.Accounts))
	stakeSupply := sdk.NewInt(simState.InitialStake * simState.NumBonded)
	qsrSupply := sdk.NewInt(simState.InitialStake * numAccs)
	supply := sdk.NewCoins(
		sdk.NewCoin(sdk.DefaultBondDenom, stakeSupply),
		sdk.NewCoin("QSR", qsrSupply),
	)

	bankGenesis := banktypes.GenesisState{
		Params: banktypes.Params{
			SendEnabled:        sendEnabledParams,
			DefaultSendEnabled: defaultSendEnabledParam,
		},
		Balances: RandomGenesisBalances(simState),
		Supply:   supply,
	}

	simState.GenState[banktypes.ModuleName] = simState.Cdc.MustMarshalJSON(&bankGenesis)
}

func RandomGenesisBalances(simState *module.SimulationState) []banktypes.Balance {
	genesisBalances := []banktypes.Balance{}

	for _, acc := range simState.Accounts {
		genesisBalances = append(genesisBalances, banktypes.Balance{
			Address: acc.Address.String(),
			Coins:   sdk.NewCoins(sdk.NewCoin("QSR", sdk.NewInt(simState.InitialStake))),
		})
	}

	return genesisBalances
}

func RandomGenesisSendParams(r *rand.Rand) banktypes.SendEnabledParams {
	params := banktypes.DefaultParams()
	// 90% chance of transfers being DefaultSendEnabled=true or P(a) = 0.9 for success
	// 50% of the time add an additional denom specific record (P(b) = 0.475 = 0.5 * 0.95)
	if r.Int63n(101) <= 50 {
		// set send enabled 95% of the time
		bondEnabled := r.Int63n(101) <= 95
		params = params.SetSendEnabledParam(
			"QSR",
			bondEnabled)
	}

	// overall probability of enabled for bond denom is 94.75% (P(a)+P(b) - P(a)*P(b))
	return params.SendEnabled
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

	var weightMsgClaimRewards int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgClaimRewards, &weightMsgClaimRewards, nil,
		func(_ *rand.Rand) {
			weightMsgClaimRewards = defaultWeightMsgClaimRewards
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgClaimRewards,
		qbanksimulation.SimulateMsgClaimRewards(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgRequestWithdrawAll int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgRequestWithdrawAll, &weightMsgRequestWithdrawAll, nil,
		func(_ *rand.Rand) {
			weightMsgRequestWithdrawAll = defaultWeightMsgRequestWithdrawAll
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgRequestWithdrawAll,
		qbanksimulation.SimulateMsgRequestWithdrawAll(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}
