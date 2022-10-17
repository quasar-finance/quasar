package keeper

import (
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	lockuptypes "github.com/quasarlabs/quasarnode/osmosis/lockup/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (k Keeper) TransmitIbcCreatePool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolParams *gammbalancer.PoolParams,
	poolAssets []gammbalancer.PoolAsset,
	futurePoolGovernor string) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0,"", "", err
	}

	msgs := []sdk.Msg{
		&gammbalancer.MsgCreateBalancerPool{
			Sender:             iaResp.InterchainAccountAddress,
			PoolParams:         poolParams,
			PoolAssets:         poolAssets,
			FuturePoolGovernor: futurePoolGovernor,
		},
	}
	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcJoinPool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareOutAmount sdk.Int,
	tokenInMaxs []sdk.Coin) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgJoinPool{
			Sender:         iaResp.InterchainAccountAddress,
			PoolId:         poolId,
			ShareOutAmount: shareOutAmount,
			TokenInMaxs:    tokenInMaxs,
		},
	}
	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcExitPool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareInAmount sdk.Int,
	tokenOutMins []sdk.Coin) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0,"", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgExitPool{
			Sender:        iaResp.InterchainAccountAddress,
			PoolId:        poolId,
			ShareInAmount: shareInAmount,
			TokenOutMins:  tokenOutMins,
		},
	}
	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcJoinSwapExternAmountIn(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	tokenIn sdk.Coin,
	shareOutMinAmount sdk.Int,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgJoinSwapExternAmountIn{
			Sender:            iaResp.InterchainAccountAddress,
			PoolId:            poolId,
			TokenIn:           tokenIn,
			ShareOutMinAmount: shareOutMinAmount,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcExitSwapExternAmountOut(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	tokenOut sdk.Coin,
	shareInMaxAmount sdk.Int,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgExitSwapExternAmountOut{
			Sender:           iaResp.InterchainAccountAddress,
			PoolId:           poolId,
			TokenOut:         tokenOut,
			ShareInMaxAmount: shareInMaxAmount,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcJoinSwapShareAmountOut(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	tokenInDenom string,
	shareOutAmount sdk.Int,
	tokenInMaxAmount sdk.Int,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgJoinSwapShareAmountOut{
			Sender:           iaResp.InterchainAccountAddress,
			PoolId:           poolId,
			TokenInDenom:     tokenInDenom,
			ShareOutAmount:   shareOutAmount,
			TokenInMaxAmount: tokenInMaxAmount,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcExitSwapShareAmountIn(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	tokenOutDenom string,
	shareInAmount sdk.Int,
	tokenOutMinAmount sdk.Int,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgExitSwapShareAmountIn{
			Sender:            iaResp.InterchainAccountAddress,
			PoolId:            poolId,
			TokenOutDenom:     tokenOutDenom,
			ShareInAmount:     shareInAmount,
			TokenOutMinAmount: tokenOutMinAmount,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcLockTokens(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	duration time.Duration,
	coins sdk.Coins,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&lockuptypes.MsgLockTokens{
			Owner:    iaResp.InterchainAccountAddress,
			Duration: duration,
			Coins:    coins,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcBeginUnlocking(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	id uint64,
	coins sdk.Coins,
) (uint64, string, string, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, "", "", err
	}

	msgs := []sdk.Msg{
		&lockuptypes.MsgBeginUnlocking{
			Owner: iaResp.InterchainAccountAddress,
			ID:    id,
			Coins: coins,
		},
	}

	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}
