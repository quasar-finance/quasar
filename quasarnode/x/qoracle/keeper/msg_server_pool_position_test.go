package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
)

func TestPoolPositionMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	expected := &types.MsgCreatePoolPosition{Creator: creator}
	_, err := srv.CreatePoolPosition(wctx, expected)
	require.NoError(t, err)
	rst, found := k.GetPoolPosition(ctx)
	require.True(t, found)
	require.Equal(t, expected.Creator, rst.Creator)
}

func TestPoolPositionMsgServerUpdate(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgUpdatePoolPosition
		err     error
	}{
		{
			desc:    "Completed",
			request: &types.MsgUpdatePoolPosition{Creator: creator},
		},
		{
			desc:    "Unauthorized",
			request: &types.MsgUpdatePoolPosition{Creator: "B"},
			err:     sdkerrors.ErrUnauthorized,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolPosition{Creator: creator}
			_, err := srv.CreatePoolPosition(wctx, expected)
			require.NoError(t, err)

			_, err = srv.UpdatePoolPosition(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				rst, found := k.GetPoolPosition(ctx)
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
			desc:    "Completed",
			request: &types.MsgDeletePoolPosition{Creator: creator},
		},
		{
			desc:    "Unauthorized",
			request: &types.MsgDeletePoolPosition{Creator: "B"},
			err:     sdkerrors.ErrUnauthorized,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolPosition(wctx, &types.MsgCreatePoolPosition{Creator: creator})
			require.NoError(t, err)
			_, err = srv.DeletePoolPosition(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				_, found := k.GetPoolPosition(ctx)
				require.False(t, found)
			}
		})
	}
}
