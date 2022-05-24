//go:build !prod

package keeper

import (
	"testing"
	"time"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"

	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

const (
	owner        string = "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
	connectionId string = "connection-0"
)

var testHooksState map[string]bool

func init() {
	testHooksState = make(map[string]bool)

	scenarios["registerIca"] = testRegisterIca
	scenarios["createPool"] = testCreatePool
	scenarios["createPoolChecks"] = testCreatePoolChecks
	scenarios["createPoolTimeout"] = testCreatePoolTimeout
	scenarios["createPoolTimeoutChecks"] = testCreatePoolTimeoutChecks
	scenarios["joinPool"] = testJoinPool
	scenarios["joinPoolChecks"] = testJoinPoolChecks
	scenarios["joinPoolTimeout"] = testJoinPoolTimeout
	scenarios["joinPoolTimeoutChecks"] = testJoinPoolTimeoutChecks
	scenarios["exitPool"] = testExitPool
	scenarios["exitPoolChecks"] = testExitPoolChecks
	scenarios["exitPoolTimeout"] = testExitPoolTimeout
	scenarios["exitPoolTimeoutChecks"] = testExitPoolTimeoutChecks
}

func createTestPoolParams() *gammbalancer.PoolParams {
	swapFee, err := sdk.NewDecFromStr("0.01")
	if err != nil {
		panic(err)
	}

	exitFee, err := sdk.NewDecFromStr("0.01")
	if err != nil {
		panic(err)
	}

	return &gammbalancer.PoolParams{
		SwapFee: swapFee,
		ExitFee: exitFee,
	}
}

func createTestPoolAssets() []gammtypes.PoolAsset {
	return []gammtypes.PoolAsset{
		{
			Weight: sdk.NewInt(100),
			Token:  sdk.NewCoin("uatom", sdk.NewInt(10000)),
		},
		{
			Weight: sdk.NewInt(100),
			Token:  sdk.NewCoin("uosmo", sdk.NewInt(10000)),
		},
	}
}

func joinPoolTestCoins() []sdk.Coin {
	return []sdk.Coin{
		sdk.NewCoin("uatom", sdk.NewInt(1000)),
		sdk.NewCoin("uosmo", sdk.NewInt(1000)),
	}
}

func ensureIcaRegistered(ctx sdk.Context, k *Keeper, owner string, connectionId string) error {
	var err error

	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return status.Errorf(codes.InvalidArgument, "could not generate port for address: %s", err)
	}

	_, found := k.icaControllerKeeper.GetOpenActiveChannel(ctx, connectionId, portID)
	if !found {
		err = k.RegisterInterchainAccount(ctx, connectionId, owner)
		if err != nil {
			return err
		}
	}

	return nil
}

func testRegisterIca(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		err := ensureIcaRegistered(ctx, k, owner, connectionId)
		require.NoError(t, err)
	}
}

func testCreatePool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
			testHooksState["testCreatePool_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
			testHooksState["testCreatePool_hook2"] = true
		})

		timestamp := uint64(99999999999999)
		futureGovernor := "168h"
		poolParams := createTestPoolParams()
		poolAssets := createTestPoolAssets()

		err = k.TransmitIbcCreatePool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolParams,
			poolAssets,
			futureGovernor,
		)
		require.NoError(t, err)
	}
}

func testCreatePoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testCreatePool_hook1"])
		require.True(t, testHooksState["testCreatePool_hook2"])
	}
}

func testCreatePoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) {
			testHooksState["testCreatePoolTimeout_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) {
			testHooksState["testCreatePoolTimeout_hook2"] = true
		})

		timestamp := uint64(99999999999999)
		futureGovernor := "168h"

		poolParams := createTestPoolParams()
		poolAssets := createTestPoolAssets()

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcCreatePool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolParams,
			poolAssets,
			futureGovernor,
		)
		require.NoError(t, err)
	}
}

func testCreatePoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testCreatePoolTimeout_hook1"])
		require.True(t, testHooksState["testCreatePoolTimeout_hook2"])
	}
}

func testJoinPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksAckMsgJoinPool(func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) {
			testHooksState["testJoinPool_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksAckMsgJoinPool(func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) {
			testHooksState["testJoinPool_hook2"] = true
		})

		poolId := uint64(1)
		timestamp := uint64(99999999999999)
		shares, ok := sdk.NewIntFromString("1000000000000000000")
		testCoins := joinPoolTestCoins()
		require.True(t, ok)

		err = k.TransmitIbcJoinPool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			shares,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testJoinPoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinPool_hook1"])
		require.True(t, testHooksState["testJoinPool_hook2"])
	}
}

func testJoinPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinPool(func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool]) {
			testHooksState["testJoinPoolTimeout_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinPool(func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool]) {
			testHooksState["testJoinPoolTimeout_hook2"] = true
		})

		poolId := uint64(1)
		timestamp := uint64(99999999999999)
		shares, ok := sdk.NewIntFromString("1000000000000000000")
		testCoins := joinPoolTestCoins()
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcJoinPool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			shares,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testJoinPoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinPoolTimeout_hook1"])
		require.True(t, testHooksState["testJoinPoolTimeout_hook2"])
	}
}

func testExitPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) {
			testHooksState["testExitPool_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) {
			testHooksState["testExitPool_hook2"] = true
		})

		poolId := uint64(1)
		timestamp := uint64(99999999999999)
		shares, ok := sdk.NewIntFromString("1000000000000000000")
		testCoins := joinPoolTestCoins()
		require.True(t, ok)

		err = k.TransmitIbcExitPool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			shares,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testExitPoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitPool_hook1"])
		require.True(t, testHooksState["testExitPool_hook2"])
	}
}

func testExitPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hooks
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitPool(func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool]) {
			testHooksState["testExitPoolTimeout_hook1"] = true
		})
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitPool(func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool]) {
			testHooksState["testExitPoolTimeout_hook2"] = true
		})

		poolId := uint64(1)
		timestamp := uint64(99999999999999)
		shares, ok := sdk.NewIntFromString("1000000000000000000")
		testCoins := joinPoolTestCoins()
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcExitPool(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			shares,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testExitPoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitPoolTimeout_hook1"])
		require.True(t, testHooksState["testExitPoolTimeout_hook2"])
	}
}
