package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	balancer "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	gamm_types "github.com/abag/quasarnode/x/gamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	osmosis_gamm_types "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/stretchr/testify/require"
)

func sampleBalancerPool() (res balancer.BalancerPool) {
	res.Address = "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t"
	res.Id = 1
	res.PoolParams = balancer.BalancerPoolParams{
		SwapFee: sdk.NewDecWithPrec(1, 2),
		ExitFee: sdk.NewDecWithPrec(1, 2),
	}
	res.FuturePoolGovernor = "24h"
	res.TotalShares = sdk.NewCoin(osmosis_gamm_types.GetPoolShareDenom(res.Id), sdk.ZeroInt())
	res.PoolAssets = []gamm_types.PoolAsset{
		{
			Weight: sdk.NewInt(100).MulRaw(osmosis_gamm_types.GuaranteedWeightPrecision),
			Token:  sdk.NewCoin("test", sdk.NewInt(100)),
		},
		{
			Weight: sdk.NewInt(100).MulRaw(osmosis_gamm_types.GuaranteedWeightPrecision),
			Token:  sdk.NewCoin("test2", sdk.NewInt(100)),
		},
	}
	gamm_types.SortPoolAssetsByDenom(res.PoolAssets)
	res.TotalWeight = sdk.ZeroInt()
	for _, asset := range res.PoolAssets {
		res.TotalWeight = res.TotalWeight.Add(asset.Weight)
	}

	return
}

func TestMsgCreatePoolInfo_ValidateBasic(t *testing.T) {
	validPool := sampleBalancerPool()

	tests := []struct {
		name string
		msg  MsgCreatePoolInfo
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreatePoolInfo{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgCreatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Info:            &validPool,
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgCreatePoolInfo{
				Creator:         sample.AccAddress(),
				Info:            &validPool,
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil Info",
			msg: MsgCreatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Pool",
			msg: MsgCreatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Info:            &balancer.BalancerPool{},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgCreatePoolInfo{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Info:    &validPool,
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgUpdatePoolInfo_ValidateBasic(t *testing.T) {
	validPool := sampleBalancerPool()

	tests := []struct {
		name string
		msg  MsgUpdatePoolInfo
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdatePoolInfo{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgUpdatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Info:            &validPool,
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgUpdatePoolInfo{
				Creator:         sample.AccAddress(),
				Info:            &validPool,
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil Info",
			msg: MsgUpdatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Pool",
			msg: MsgUpdatePoolInfo{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Info:            &balancer.BalancerPool{},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgUpdatePoolInfo{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Info:    &validPool,
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgDeletePoolInfo_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgDeletePoolInfo
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgDeletePoolInfo{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgDeletePoolInfo{
				Creator: sample.AccAddress(),
				PoolId:  "1",
			},
		}, {
			name: "empty PoolId",
			msg: MsgDeletePoolInfo{
				Creator: sample.AccAddress(),
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}
