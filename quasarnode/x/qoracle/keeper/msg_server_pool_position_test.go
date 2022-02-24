package keeper_test

import (
	"strconv"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
)

// Prevent strconv unused error
var _ = strconv.IntSize

func TestPoolPositionMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	for i := 0; i < 5; i++ {
		expected := &types.MsgCreatePoolPosition{Creator: creator,
			PoolId: strconv.Itoa(i),
		}
		_, err := srv.CreatePoolPosition(wctx, expected)
		require.NoError(t, err)
		rst, found := k.GetPoolPosition(ctx,
			expected.PoolId,
		)
		require.True(t, found)
		require.Equal(t, expected.Creator, rst.Creator)
	}
}

func TestPoolPositionMsgServerUpdate(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgUpdatePoolPosition
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgUpdatePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(0),
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgUpdatePoolPosition{Creator: "B",
				PoolId: strconv.Itoa(0),
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgUpdatePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(100000),
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(0),
			}
			_, err := srv.CreatePoolPosition(wctx, expected)
			require.NoError(t, err)

			_, err = srv.UpdatePoolPosition(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				rst, found := k.GetPoolPosition(ctx,
					expected.PoolId,
				)
				require.True(t, found)
				require.Equal(t, expected.Creator, rst.Creator)
			}
		})
	}
}

func TestPoolPositionMsgServerDelete(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgDeletePoolPosition
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgDeletePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(0),
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgDeletePoolPosition{Creator: "B",
				PoolId: strconv.Itoa(0),
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgDeletePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(100000),
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolPosition(wctx, &types.MsgCreatePoolPosition{Creator: creator,
				PoolId: strconv.Itoa(0),
			})
			require.NoError(t, err)
			_, err = srv.DeletePoolPosition(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				_, found := k.GetPoolPosition(ctx,
					tc.request.PoolId,
				)
				require.False(t, found)
			}
		})
	}
}
