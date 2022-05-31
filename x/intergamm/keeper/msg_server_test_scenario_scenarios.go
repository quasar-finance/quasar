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
	lockuptypes "github.com/osmosis-labs/osmosis/v7/x/lockup/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

const (
	owner        string = "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
	connectionId string = "connection-0"
	poolId              = uint64(1)
	timestamp           = uint64(99999999999999)
)

type HookState struct {
	Called bool
	Error  string
}

var testHooksState map[string]HookState

func init() {
	testHooksState = make(map[string]HookState)

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
	scenarios["joinSwapExternAmountIn"] = testJoinSwapExternAmountIn
	scenarios["joinSwapExternAmountInChecks"] = testJoinSwapExternAmountInChecks
	scenarios["joinSwapExternAmountInTimeout"] = testJoinSwapExternAmountInTimeout
	scenarios["joinSwapExternAmountInTimeoutChecks"] = testJoinSwapExternAmountInTimeoutChecks
	scenarios["exitSwapExternAmountOut"] = testExitSwapExternAmountOut
	scenarios["exitSwapExternAmountOutChecks"] = testExitSwapExternAmountOutChecks
	scenarios["exitSwapExternAmountOutTimeout"] = testExitSwapExternAmountOutTimeout
	scenarios["exitSwapExternAmountOutTimeoutChecks"] = testExitSwapExternAmountOutTimeoutChecks
	scenarios["joinSwapShareAmountOut"] = testJoinSwapShareAmountOut
	scenarios["joinSwapShareAmountOutChecks"] = testJoinSwapShareAmountOutChecks
	scenarios["joinSwapShareAmountOutTimeout"] = testJoinSwapShareAmountOutTimeout
	scenarios["joinSwapShareAmountOutTimeoutChecks"] = testJoinSwapShareAmountOutTimeoutChecks
	scenarios["exitSwapShareAmountIn"] = testExitSwapShareAmountIn
	scenarios["exitSwapShareAmountInChecks"] = testExitSwapShareAmountInChecks
	scenarios["exitSwapShareAmountInTimeout"] = testExitSwapShareAmountInTimeout
	scenarios["exitSwapShareAmountInTimeoutChecks"] = testExitSwapShareAmountInTimeoutChecks
	scenarios["lockTokens"] = testLockTokens
	scenarios["lockTokensChecks"] = testLockTokensChecks
	scenarios["lockTokensTimeout"] = testLockTokensTimeout
	scenarios["lockTokensTimeoutChecks"] = testLockTokensTimeoutChecks
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

func joinSwapExternAmountInTestCoin() sdk.Coin {
	return sdk.NewCoin("uatom", sdk.NewInt(1000))
}

func lockTokensTestCoins() []sdk.Coin {
	return []sdk.Coin{
		sdk.NewCoin("gamm/pool/1", sdk.NewInt(42000)),
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

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(ctx sdk.Context, ex types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
			testHooksState["testCreatePool_hook"] = HookState{
				Called: true,
			}
		})

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
		require.True(t, testHooksState["testCreatePool_hook"].Called)
		require.Empty(t, testHooksState["testCreatePool_hook"].Error)
	}
}

func testCreatePoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) {
			testHooksState["testCreatePoolTimeout_hook"] = HookState{
				Called: true,
			}
		})

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
		require.True(t, testHooksState["testCreatePoolTimeout_hook"].Called)
	}
}

func testJoinPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgJoinPool(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) {
			testHooksState["testJoinPool_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testCoins := joinPoolTestCoins()
		shares, ok := sdk.NewIntFromString("1000000000000000000")
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
		require.True(t, testHooksState["testJoinPool_hook"].Called)
		require.Empty(t, testHooksState["testJoinPool_hook"].Error)
	}
}

func testJoinPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinPool]) {
			testHooksState["testJoinPoolTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testCoins := joinPoolTestCoins()
		shares, ok := sdk.NewIntFromString("1000000000000000000")
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
		require.True(t, testHooksState["testJoinPoolTimeout_hook"].Called)
	}
}

func testExitPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) {
			testHooksState["testExitPool_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testCoins := []sdk.Coin{}
		shares, ok := sdk.NewIntFromString("1000000000000000000")
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
		require.True(t, testHooksState["testExitPool_hook"].Called)
		require.Empty(t, testHooksState["testExitPool_hook"].Error)
	}
}

func testExitPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitPool]) {
			testHooksState["testExitPoolTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testCoins := joinPoolTestCoins()
		shares, ok := sdk.NewIntFromString("1000000000000000000")
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
		require.True(t, testHooksState["testExitPoolTimeout_hook"].Called)
	}
}

func testJoinSwapExternAmountIn(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgJoinSwapExternAmountIn(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]) {
			testHooksState["testJoinSwapExternAmountIn_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testCoin := joinSwapExternAmountInTestCoin()
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		err = k.TransmitIbcJoinSwapExternAmountIn(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testCoin,
			shares,
		)
		require.NoError(t, err)
	}
}

func testJoinSwapExternAmountInChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapExternAmountIn_hook"].Called)
		require.Empty(t, testHooksState["testJoinSwapExternAmountIn_hook"].Error)
	}
}

func testJoinSwapExternAmountInTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapExternAmountIn(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]) {
			testHooksState["testJoinSwapExternAmountInTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testCoin := joinSwapExternAmountInTestCoin()
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcJoinSwapExternAmountIn(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testCoin,
			shares,
		)
		require.NoError(t, err)
	}
}

func testJoinSwapExternAmountInTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapExternAmountInTimeout_hook"].Called)
	}
}

func testExitSwapExternAmountOut(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgExitSwapExternAmountOut(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]) {
			testHooksState["testExitSwapExternAmountOut_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testCoin := sdk.NewCoin("uatom", sdk.NewInt(1))
		shares, ok := sdk.NewIntFromString("10000000000000000")
		require.True(t, ok)

		err = k.TransmitIbcExitSwapExternAmountOut(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testCoin,
			shares,
		)
		require.NoError(t, err)
	}
}

func testExitSwapExternAmountOutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapExternAmountOut_hook"].Called)
		require.Empty(t, testHooksState["testExitSwapExternAmountOut_hook"].Error)
	}
}

func testExitSwapExternAmountOutTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapExternAmountOut(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]) {
			testHooksState["testExitSwapExternAmountOutTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testCoin := sdk.Coin{}
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcExitSwapExternAmountOut(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testCoin,
			shares,
		)
		require.NoError(t, err)
	}
}

func testExitSwapExternAmountOutTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapExternAmountOutTimeout_hook"].Called)
	}
}

func testJoinSwapShareAmountOut(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgJoinSwapShareAmountOut(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]) {
			testHooksState["testJoinSwapShareAmountOut_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testDenom := "uatom"
		testCoinAmount := sdk.NewInt(1000)
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		err = k.TransmitIbcJoinSwapShareAmountOut(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testDenom,
			shares,
			testCoinAmount,
		)
		require.NoError(t, err)
	}
}

func testJoinSwapShareAmountOutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapShareAmountOut_hook"].Called)
		require.Empty(t, testHooksState["testJoinSwapShareAmountOut_hook"].Error)
	}
}

func testJoinSwapShareAmountOutTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapShareAmountOut(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]) {
			testHooksState["testJoinSwapShareAmountOutTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testDenom := "uatom"
		testCoinAmount := sdk.NewInt(1000)
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcJoinSwapShareAmountOut(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testDenom,
			shares,
			testCoinAmount,
		)
		require.NoError(t, err)
	}
}

func testJoinSwapShareAmountOutTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapShareAmountOutTimeout_hook"].Called)
	}
}

func testExitSwapShareAmountIn(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgExitSwapShareAmountIn(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]) {
			testHooksState["testExitSwapShareAmountIn_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		testDenom := "uatom"
		testCoinAmount := sdk.NewInt(1)
		shares, ok := sdk.NewIntFromString("10000000000000000")
		require.True(t, ok)

		err = k.TransmitIbcExitSwapShareAmountIn(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testDenom,
			shares,
			testCoinAmount,
		)
		require.NoError(t, err)
	}
}

func testExitSwapShareAmountInChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapShareAmountIn_hook"].Called)
		require.Empty(t, testHooksState["testExitSwapShareAmountIn_hook"].Error)
	}
}

func testExitSwapShareAmountInTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapShareAmountIn(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]) {
			testHooksState["testExitSwapShareAmountInTimeout_hook"] = HookState{
				Called: true,
			}
		})

		testDenom := "uatom"
		testCoinAmount := sdk.NewInt(1000)
		shares, ok := sdk.NewIntFromString("500000000000000000")
		require.True(t, ok)

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcExitSwapShareAmountIn(
			ctx,
			owner,
			connectionId,
			timestamp,
			poolId,
			testDenom,
			shares,
			testCoinAmount,
		)
		require.NoError(t, err)
	}
}

func testExitSwapShareAmountInTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapShareAmountInTimeout_hook"].Called)
	}
}

func testLockTokens(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksAckMsgLockTokens(func(ctx sdk.Context, ex types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) {
			testHooksState["testLockTokens_hook"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
		})

		lockupPeriod := 1 * time.Hour
		testCoins := lockTokensTestCoins()

		err = k.TransmitIbcLockTokens(
			ctx,
			owner,
			connectionId,
			timestamp,
			lockupPeriod,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testLockTokensChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testLockTokens_hook"].Called)
		require.Empty(t, testHooksState["testLockTokens_hook"].Error)
	}
}

func testLockTokensTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		var err error

		// Setup hook
		k.Hooks.Osmosis.AddHooksTimeoutMsgLockTokens(func(ctx sdk.Context, ex types.TimeoutExchange[*lockuptypes.MsgLockTokens]) {
			testHooksState["testLockTokensTimeout_hook"] = HookState{
				Called: true,
			}
		})

		lockupPeriod := 1 * time.Hour
		testCoins := lockTokensTestCoins()

		// Replace timeout to trigger timeout hooks
		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
		defer func() {
			DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
		}()

		err = k.TransmitIbcLockTokens(
			ctx,
			owner,
			connectionId,
			timestamp,
			lockupPeriod,
			testCoins,
		)
		require.NoError(t, err)
	}
}

func testLockTokensTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testLockTokensTimeout_hook"].Called)
	}
}
