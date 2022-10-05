package simulation

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func SimulateMsgTransmitICATransfer(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)
		msg := &types.MsgTransmitICATransfer{
			IcaOwnerAddress: simAccount.Address.String(),
		}

		// TODO: Handling the TransmitICATransfer simulation

		return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "TransmitICATransfer simulation not implemented"), nil, nil
	}
}
