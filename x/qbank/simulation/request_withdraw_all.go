package simulation

import (
	"math/rand"

	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
)

func SimulateMsgRequestWithdrawAll(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)
		msg := &types.MsgRequestWithdrawAll{
			Creator: simAccount.Address.String(),
		}

		// TODO: Handling the RequestWithdrawAll simulation

		return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "RequestWithdrawAll simulation not implemented"), nil, nil
	}
}
