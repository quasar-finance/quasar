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

func TestPoolPositionMsgServerCreate(t *testing.T) {
	k, ctx := keepertest.QoracleKeeper(t)
	srv := keeper.NewMsgServerImpl(*k)
	wctx := sdk.WrapSDKContext(ctx)
	creator := "A"
	for i := 0; i < 5; i++ {
		expected := &types.MsgCreatePoolPosition{Creator: creator,
			PoolId: fmt.Sprintf("%d", i),
			Metrics: &types.PoolMetrics{
				HighestAPY: sdk.NewDec(int64(i)).String(),
				TVL:        sdk.NewDecCoin("usd", sdk.NewInt(int64(i))).String(),
				GaugeAPYs: []*types.GaugeAPY{
					&types.GaugeAPY{GaugeId: 3*uint64(i) + 1, Duration: "1s", APY: "1.1"},
					&types.GaugeAPY{GaugeId: 3*uint64(i) + 2, Duration: "2s", APY: "1.2"},
				},
			},
			LastUpdatedTime: 1 + uint64(i),
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
				PoolId: "1",
				Metrics: &types.PoolMetrics{
					HighestAPY: "1.2",
					TVL:        "200.5usd",
					GaugeAPYs: []*types.GaugeAPY{
						&types.GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
						&types.GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
					},
				},
				LastUpdatedTime: 2,
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgUpdatePoolPosition{Creator: "B",
				PoolId: "1",
				Metrics: &types.PoolMetrics{
					HighestAPY: "1.2",
					TVL:        "200.5usd",
					GaugeAPYs: []*types.GaugeAPY{
						&types.GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
						&types.GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
					},
				},
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgUpdatePoolPosition{Creator: creator,
				PoolId: "10",
				Metrics: &types.PoolMetrics{
					HighestAPY: "1.2",
					TVL:        "200.5usd",
					GaugeAPYs: []*types.GaugeAPY{
						&types.GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
						&types.GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
					},
				},
				LastUpdatedTime: 2,
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)
			expected := &types.MsgCreatePoolPosition{Creator: creator,
				PoolId: "1",
				Metrics: &types.PoolMetrics{
					HighestAPY: "1.1",
					TVL:        "100.5usd",
					GaugeAPYs: []*types.GaugeAPY{
						&types.GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
						&types.GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
					},
				},
				LastUpdatedTime: 1,
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
				PoolId: "1",
			},
		},
		{
			desc: "Unauthorized",
			request: &types.MsgDeletePoolPosition{Creator: "B",
				PoolId: "1",
			},
			err: sdkerrors.ErrUnauthorized,
		},
		{
			desc: "KeyNotFound",
			request: &types.MsgDeletePoolPosition{Creator: creator,
				PoolId: "10",
			},
			err: sdkerrors.ErrKeyNotFound,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			k, ctx := keepertest.QoracleKeeper(t)
			srv := keeper.NewMsgServerImpl(*k)
			wctx := sdk.WrapSDKContext(ctx)

			_, err := srv.CreatePoolPosition(wctx, &types.MsgCreatePoolPosition{Creator: creator,
				PoolId: "1",
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
