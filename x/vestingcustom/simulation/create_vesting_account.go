package simulation

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"quasartodel/x/vestingcustom/keeper"
	"quasartodel/x/vestingcustom/types"
)

func SimulateMsgCreateVestingAccount(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)
		msg := &types.MsgCreateVestingAccount{
			Creator: simAccount.Address.String(),
		}

		// TODO: Handling the CreateVestingAccount simulation

		return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "CreateVestingAccount simulation not implemented"), nil, nil
	}
}
