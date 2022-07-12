package simulation

import (
	"math/rand"

	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
)

func SimulateMsgAddDenomPriceMapping(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)
		msg := &types.MsgAddDenomPriceMapping{
			Creator: simAccount.Address.String(),
		}

		// TODO: Handling the AddDenomPriceMapping simulation

		return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "AddDenomPriceMapping simulation not implemented"), nil, nil
	}
}
