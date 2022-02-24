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

func TestPoolSpotPriceMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	for i := 0; i < 5; i++ {
		expected := &types.MsgCreatePoolSpotPrice{Creator: creator,
			PoolId:   strconv.Itoa(i),
			DenomIn:  strconv.Itoa(i),
			DenomOut: strconv.Itoa(i),
		}
		_, err := srv.CreatePoolSpotPrice(wctx, expected)
		require.NoError(t, err)
		rst, found := k.GetPoolSpotPrice(ctx,
			expected.PoolId,
			expected.DenomIn,
			expected.DenomOut,
		)
		require.True(t, found)
		require.Equal(t, expected.Creator, rst.Creator)
	}
}

func TestPoolSpotPriceMsgServerUpdate(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgUpdatePoolSpotPrice
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgUpdatePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgUpdatePoolSpotPrice{Creator: "B",
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgUpdatePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(100000),
				DenomIn:  strconv.Itoa(100000),
				DenomOut: strconv.Itoa(100000),
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			}
			_, err := srv.CreatePoolSpotPrice(wctx, expected)
			require.NoError(t, err)

			_, err = srv.UpdatePoolSpotPrice(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				rst, found := k.GetPoolSpotPrice(ctx,
					expected.PoolId,
					expected.DenomIn,
					expected.DenomOut,
				)
				require.True(t, found)
				require.Equal(t, expected.Creator, rst.Creator)
			}
		})
	}
}

func TestPoolSpotPriceMsgServerDelete(t *testing.T) {
	creator := "A"

	for _, tc := range []struct {
		desc    string
		request *types.MsgDeletePoolSpotPrice
		err     error
	}{
		{
			desc: "Completed",
			request: &types.MsgDeletePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgDeletePoolSpotPrice{Creator: "B",
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgDeletePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(100000),
				DenomIn:  strconv.Itoa(100000),
				DenomOut: strconv.Itoa(100000),
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolSpotPrice(wctx, &types.MsgCreatePoolSpotPrice{Creator: creator,
				PoolId:   strconv.Itoa(0),
				DenomIn:  strconv.Itoa(0),
				DenomOut: strconv.Itoa(0),
			})
			require.NoError(t, err)
			_, err = srv.DeletePoolSpotPrice(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				_, found := k.GetPoolSpotPrice(ctx,
					tc.request.PoolId,
					tc.request.DenomIn,
					tc.request.DenomOut,
				)
				require.False(t, found)
			}
		})
	}
}
