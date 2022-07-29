package simulation

import (
	"math/rand"

	"github.com/quasarlabs/quasarnode/x/qbank/keeper"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	simappparams "github.com/cosmos/cosmos-sdk/simapp/params"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
)

func SimulateMsgRequestDeposit(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)

		spendable := bk.SpendableCoins(ctx, simAccount.Address)
		coins := simtypes.RandSubsetCoins(r, spendable)

		if coins.Len() == 0 {
			return simtypes.NoOpMsg(types.ModuleName, types.TypeMsgRequestDeposit, "no spendable coins"), nil, nil
		}

		if err := bk.IsSendEnabledCoins(ctx, coins...); err != nil {
			return simtypes.NoOpMsg(types.ModuleName, types.TypeMsgRequestDeposit, err.Error()), nil, nil
		}

		msg := &types.MsgRequestDeposit{
			Creator:      simAccount.Address.String(),
			RiskProfile:  RandRiskProfile(r),
			VaultID:      "orion",
			Coin:         sdk.NewCoin("QSR", sdk.NewInt(42)), // FIXME coins.AmountOf("QSR") triggers insufficient balance
			LockupPeriod: RandLockupTime(r),
		}

		txCtx := simulation.OperationInput{
			R:             r,
			App:           app,
			TxGen:         simappparams.MakeTestEncodingConfig().TxConfig,
			Cdc:           nil,
			Msg:           msg,
			MsgType:       msg.Type(),
			Context:       ctx,
			SimAccount:    simAccount,
			AccountKeeper: ak,
			Bankkeeper:    bk,
			ModuleName:    types.ModuleName,
		}

		return simulation.GenAndDeliverTxWithRandFees(txCtx)
	}
}

func RandRiskProfile(r *rand.Rand) string {
	n := r.Intn(3)
	switch n {
	case 0:
		return "LOW"
	case 1:
		return "MID"
	default:
		return "HIGH"
	}
}

func RandLockupTime(r *rand.Rand) types.LockupTypes {
	n := r.Intn(5-1) + 1

	return types.LockupTypes(n)
}
