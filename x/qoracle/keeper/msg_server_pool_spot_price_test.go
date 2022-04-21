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

func TestPoolSpotPriceMsgServerCreate(t *testing.T) {
	ctx, k := keepertest.NewTestSetup(t).GetQoracleKeeper()
	srv := keeper.NewMsgServerImpl(k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	for i := 0; i < 5; i++ {
		expected := &types.MsgCreatePoolSpotPrice{Creator: creator,
			PoolId:          fmt.Sprintf("%d", i),
			DenomIn:         fmt.Sprintf("abc%d", i),
			DenomOut:        fmt.Sprintf("cba%d", i),
			Price:           fmt.Sprintf("%f", 1.5*float32(i)),
			LastUpdatedTime: 1 + uint64(i),
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
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 2,
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgUpdatePoolSpotPrice{Creator: "B",
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgUpdatePoolSpotPrice{Creator: creator,
				PoolId:          "10",
				DenomIn:         "xyz",
				DenomOut:        "zyx",
				Price:           "1.2",
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			ctx, k := keepertest.NewTestSetup(t).GetQoracleKeeper()
			srv := keeper.NewMsgServerImpl(k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolSpotPrice{Creator: creator,
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.1",
				LastUpdatedTime: 1,
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
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "cba",
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgDeletePoolSpotPrice{Creator: "B",
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "cba",
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgDeletePoolSpotPrice{Creator: creator,
				PoolId:   "10",
				DenomIn:  "xyz",
				DenomOut: "zyx",
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			ctx, k := keepertest.NewTestSetup(t).GetQoracleKeeper()
			srv := keeper.NewMsgServerImpl(k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolSpotPrice(wctx, &types.MsgCreatePoolSpotPrice{Creator: creator,
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.1",
				LastUpdatedTime: 1,
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
