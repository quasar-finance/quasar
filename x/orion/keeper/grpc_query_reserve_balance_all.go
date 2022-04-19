package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// ReserveBalanceAll returns the account info of orion module accounts (reserve and fee collector)
func (k Keeper) ReserveBalanceAll(goCtx context.Context, req *types.QueryReserveBalanceAllRequest) (*types.QueryReserveBalanceAllResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	k.Logger(ctx).Debug("method- ReserveBalanceAll", "req", *req)
	var ais []types.AccountInfo
	reserveAccInfo := types.AccountInfo{Name: types.OrionReserveMaccName,
		Account: k.GetBech32ReserveAccAddress(),
		Balance: k.GetAllReserveBalances(ctx)}

	mgmtFeeAccInfo := types.AccountInfo{Name: types.MgmtFeeCollectorMaccName,
		Account: k.GetBech32FeeCollectorAccAddress(types.MgmtFeeCollectorMaccName),
		Balance: k.GetFeeCollectorBalances(ctx, types.MgmtFeeCollectorMaccName)}

	perfFeeAccInfo := types.AccountInfo{Name: types.PerfFeeCollectorMaccName,
		Account: k.GetBech32FeeCollectorAccAddress(types.PerfFeeCollectorMaccName),
		Balance: k.GetFeeCollectorBalances(ctx, types.PerfFeeCollectorMaccName)}

	ais = append(ais, reserveAccInfo, mgmtFeeAccInfo, perfFeeAccInfo)

	return &types.QueryReserveBalanceAllResponse{AccInfo: ais}, nil
}
