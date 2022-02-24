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

func TestPoolRankingMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	expected := &types.MsgCreatePoolRanking{Creator: creator}
	_, err := srv.CreatePoolRanking(wctx, expected)
	require.NoError(t, err)
	rst, found := k.GetPoolRanking(ctx)
	require.True(t, found)
	require.Equal(t, expected.Creator, rst.Creator)
}

func TestPoolRankingMsgServerUpdate(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgUpdatePoolRanking
		err     error
	}{
		{
			desc:    "Completed",
			request: &types.MsgUpdatePoolRanking{Creator: creator},
		},
		{
			desc:    "Unauthorized",
			request: &types.MsgUpdatePoolRanking{Creator: "B"},
			err:     sdkerrors.ErrUnauthorized,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolRanking{Creator: creator}
			_, err := srv.CreatePoolRanking(wctx, expected)
			require.NoError(t, err)

			_, err = srv.UpdatePoolRanking(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				rst, found := k.GetPoolRanking(ctx)
				require.True(t, found)
				require.Equal(t, expected.Creator, rst.Creator)
			}
		})
	}
}

func TestPoolRankingMsgServerDelete(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgDeletePoolRanking
		err     error
	}{
		{
			desc:    "Completed",
			request: &types.MsgDeletePoolRanking{Creator: creator},
		},
		{
			desc:    "Unauthorized",
			request: &types.MsgDeletePoolRanking{Creator: "B"},
			err:     sdkerrors.ErrUnauthorized,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolRanking(wctx, &types.MsgCreatePoolRanking{Creator: creator})
			require.NoError(t, err)
			_, err = srv.DeletePoolRanking(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				_, found := k.GetPoolRanking(ctx)
				require.False(t, found)
			}
		})
	}
}
