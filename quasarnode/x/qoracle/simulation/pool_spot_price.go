package simulation

import (
	"math/rand"
	"strconv"

	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
)

// Prevent strconv unused error
var _ = strconv.IntSize

func SimulateMsgCreatePoolSpotPrice(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)

		i := r.Int()
		msg := &types.MsgCreatePoolSpotPrice{
			Creator:  simAccount.Address.String(),
			PoolId:   strconv.Itoa(i),
			DenomIn:  strconv.Itoa(i),
			DenomOut: strconv.Itoa(i),
		}

		_, found := k.GetPoolSpotPrice(ctx, msg.PoolId, msg.DenomIn, msg.DenomOut)
		if found {
			return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "PoolSpotPrice already exist"), nil, nil
		}

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      simAccount,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.NewCoins(),
			AccountKeeper:   ak,
			Bankkeeper:      bk,
		}
		return simulation.GenAndDeliverTxWithRandFees(txCtx)
	}
}

func SimulateMsgUpdatePoolSpotPrice(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		var (
			simAccount       = simtypes.Account{}
			poolSpotPrice    = types.PoolSpotPrice{}
			msg              = &types.MsgUpdatePoolSpotPrice{}
			allPoolSpotPrice = k.GetAllPoolSpotPrice(ctx)
			found            = false
		)
		for _, obj := range allPoolSpotPrice {
			simAccount, found = FindAccount(accs, obj.Creator)
			if found {
				poolSpotPrice = obj
				break
			}
		}
		if !found {
			return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "poolSpotPrice creator not found"), nil, nil
		}
		msg.Creator = simAccount.Address.String()

		msg.PoolId = poolSpotPrice.PoolId
		msg.DenomIn = poolSpotPrice.DenomIn
		msg.DenomOut = poolSpotPrice.DenomOut

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      simAccount,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.NewCoins(),
			AccountKeeper:   ak,
			Bankkeeper:      bk,
		}
		return simulation.GenAndDeliverTxWithRandFees(txCtx)
	}
}

func SimulateMsgDeletePoolSpotPrice(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		var (
			simAccount       = simtypes.Account{}
			poolSpotPrice    = types.PoolSpotPrice{}
			msg              = &types.MsgUpdatePoolSpotPrice{}
			allPoolSpotPrice = k.GetAllPoolSpotPrice(ctx)
			found            = false
		)
		for _, obj := range allPoolSpotPrice {
			simAccount, found = FindAccount(accs, obj.Creator)
			if found {
				poolSpotPrice = obj
				break
			}
		}
		if !found {
			return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "poolSpotPrice creator not found"), nil, nil
		}
		msg.Creator = simAccount.Address.String()

		msg.PoolId = poolSpotPrice.PoolId
		msg.DenomIn = poolSpotPrice.DenomIn
		msg.DenomOut = poolSpotPrice.DenomOut

		txCtx := simulation.OperationInput{
			R:               r,
			App:             app,
			TxGen:           simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:             nil,
			Msg:             msg,
			MsgType:         msg.Type(),
			Context:         ctx,
			SimAccount:      simAccount,
			ModuleName:      types.ModuleName,
			CoinsSpentInMsg: sdk.NewCoins(),
			AccountKeeper:   ak,
			Bankkeeper:      bk,
		}
		return simulation.GenAndDeliverTxWithRandFees(txCtx)
	}
}
