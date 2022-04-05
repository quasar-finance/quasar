package keeper_test

import (
	"fmt"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
)

func TestPoolInfoMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	for i := 0; i < 5; i++ {
		expected := &types.MsgCreatePoolInfo{Creator: creator,
			PoolId:          fmt.Sprintf("%d", i),
			LastUpdatedTime: 1 + uint64(i),
		}
		_, err := srv.CreatePoolInfo(wctx, expected)
		require.NoError(t, err)
		rst, found := k.GetPoolInfo(ctx,
			expected.PoolId,
		)
		require.True(t, found)
		require.Equal(t, expected.Creator, rst.Creator)
	}
}

func TestPoolInfoMsgServerUpdate(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgUpdatePoolInfo
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgUpdatePoolInfo{Creator: creator,
				PoolId:          "1",
				LastUpdatedTime: 2,
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgUpdatePoolInfo{Creator: "B",
				PoolId:          "1",
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgUpdatePoolInfo{Creator: creator,
				PoolId:          "10",
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolInfo{Creator: creator,
				PoolId:          "1",
				LastUpdatedTime: 1,
			}
			_, err := srv.CreatePoolInfo(wctx, expected)
			require.NoError(t, err)

			_, err = srv.UpdatePoolInfo(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				rst, found := k.GetPoolInfo(ctx,
					expected.PoolId,
				)
				require.True(t, found)
				require.Equal(t, expected.Creator, rst.Creator)
			}
		})
	}
}

func TestPoolInfoMsgServerDelete(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgDeletePoolInfo
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgDeletePoolInfo{Creator: creator,
				PoolId: "1",
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgDeletePoolInfo{Creator: "B",
				PoolId: "1",
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgDeletePoolInfo{Creator: creator,
				PoolId: "10",
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolInfo(wctx, &types.MsgCreatePoolInfo{Creator: creator,
				PoolId:          "1",
				LastUpdatedTime: 1,
			})
			require.NoError(t, err)
			_, err = srv.DeletePoolInfo(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				_, found := k.GetPoolInfo(ctx,
					tc.request.PoolId,
				)
				require.False(t, found)
			}
		})
	}
}
