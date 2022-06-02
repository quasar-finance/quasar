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

// Replace timeout to trigger timeout hooks in test
func swapTimeout() func() {
	tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
	DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())

	return func() {
		DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
	}
}

// RegisterIca setup

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

// CreatePool tests

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

func createPool(t *testing.T, ctx sdk.Context, k *Keeper) {
	futureGovernor := "168h"
	poolParams := createTestPoolParams()
	poolAssets := createTestPoolAssets()

	_, err := k.TransmitIbcCreatePool(
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

func testCreatePool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(ctx sdk.Context, ex types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) error {
			testHooksState["testCreatePool"] = HookState{
				Called: true,
			}
			return nil
		})

		createPool(t, ctx, k)
	}
}

func testCreatePoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testCreatePool"].Called)
		require.Empty(t, testHooksState["testCreatePool"].Error)
	}
}

func testCreatePoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) error {
			testHooksState["testCreatePoolTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		createPool(t, ctx, k)
	}
}

func testCreatePoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testCreatePoolTimeout"].Called)
	}
}

// JoinPool tests

func joinPoolTestCoins() []sdk.Coin {
	return []sdk.Coin{
		sdk.NewCoin("uatom", sdk.NewInt(1000)),
		sdk.NewCoin("uosmo", sdk.NewInt(1000)),
	}
}

func joinPool(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoins := joinPoolTestCoins()
	shares, ok := sdk.NewIntFromString("1000000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcJoinPool(
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

func testJoinPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgJoinPool(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) error {
			testHooksState["testJoinPool"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		joinPool(t, ctx, k)
	}
}

func testJoinPoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinPool"].Called)
		require.Empty(t, testHooksState["testJoinPool"].Error)
	}
}

func testJoinPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinPool]) error {
			testHooksState["testJoinPoolTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		joinPool(t, ctx, k)
	}
}

func testJoinPoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinPoolTimeout"].Called)
	}
}

// ExitPool tests

func exitPool(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoins := []sdk.Coin{}
	shares, ok := sdk.NewIntFromString("1000000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcExitPool(
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

func testExitPool(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) error {
			testHooksState["testExitPool"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		exitPool(t, ctx, k)
	}
}

func testExitPoolChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitPool"].Called)
		require.Empty(t, testHooksState["testExitPool"].Error)
	}
}

func testExitPoolTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitPool(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitPool]) error {
			testHooksState["testExitPoolTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		exitPool(t, ctx, k)
	}
}

func testExitPoolTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitPoolTimeout"].Called)
	}
}

// JoinSwapExternAmountIn tests

func joinSwapExternAmountInTestCoin() sdk.Coin {
	return sdk.NewCoin("uatom", sdk.NewInt(1000))
}

func joinSwapExternAmountIn(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoin := joinSwapExternAmountInTestCoin()
	shares, ok := sdk.NewIntFromString("500000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcJoinSwapExternAmountIn(
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

func testJoinSwapExternAmountIn(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgJoinSwapExternAmountIn(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]) error {
			testHooksState["testJoinSwapExternAmountIn"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		joinSwapExternAmountIn(t, ctx, k)
	}
}

func testJoinSwapExternAmountInChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapExternAmountIn"].Called)
		require.Empty(t, testHooksState["testJoinSwapExternAmountIn"].Error)
	}
}

func testJoinSwapExternAmountInTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapExternAmountIn(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]) error {
			testHooksState["testJoinSwapExternAmountInTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		joinSwapExternAmountIn(t, ctx, k)
	}
}

func testJoinSwapExternAmountInTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapExternAmountInTimeout"].Called)
	}
}

// ExitSwapExternAmountOut tests

func exitSwapExternAmountOut(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoin := sdk.NewCoin("uatom", sdk.NewInt(1))
	shares, ok := sdk.NewIntFromString("10000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcExitSwapExternAmountOut(
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

func testExitSwapExternAmountOut(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgExitSwapExternAmountOut(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]) error {
			testHooksState["testExitSwapExternAmountOut"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		exitSwapExternAmountOut(t, ctx, k)
	}
}

func testExitSwapExternAmountOutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapExternAmountOut"].Called)
		require.Empty(t, testHooksState["testExitSwapExternAmountOut"].Error)
	}
}

func testExitSwapExternAmountOutTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapExternAmountOut(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]) error {
			testHooksState["testExitSwapExternAmountOutTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		exitSwapExternAmountOut(t, ctx, k)
	}
}

func testExitSwapExternAmountOutTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapExternAmountOutTimeout"].Called)
	}
}

// JoinSwapShareAmountOut tests

func joinSwapShareAmountOut(t *testing.T, ctx sdk.Context, k *Keeper) {
	testDenom := "uatom"
	testCoinAmount := sdk.NewInt(1000)
	shares, ok := sdk.NewIntFromString("500000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcJoinSwapShareAmountOut(
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

func testJoinSwapShareAmountOut(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgJoinSwapShareAmountOut(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]) error {
			testHooksState["testJoinSwapShareAmountOut"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		joinSwapShareAmountOut(t, ctx, k)
	}
}

func testJoinSwapShareAmountOutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapShareAmountOut"].Called)
		require.Empty(t, testHooksState["testJoinSwapShareAmountOut"].Error)
	}
}

func testJoinSwapShareAmountOutTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapShareAmountOut(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]) error {
			testHooksState["testJoinSwapShareAmountOutTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		joinSwapShareAmountOut(t, ctx, k)
	}
}

func testJoinSwapShareAmountOutTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testJoinSwapShareAmountOutTimeout"].Called)
	}
}

// ExitSwapShareAmountIn tests

func exitSwapShareAmountIn(t *testing.T, ctx sdk.Context, k *Keeper) {
	testDenom := "uatom"
	testCoinAmount := sdk.NewInt(1)
	shares, ok := sdk.NewIntFromString("10000000000000000")
	require.True(t, ok)

	_, err := k.TransmitIbcExitSwapShareAmountIn(
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

func testExitSwapShareAmountIn(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgExitSwapShareAmountIn(func(ctx sdk.Context, ex types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]) error {
			testHooksState["testExitSwapShareAmountIn"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		exitSwapShareAmountIn(t, ctx, k)
	}
}

func testExitSwapShareAmountInChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapShareAmountIn"].Called)
		require.Empty(t, testHooksState["testExitSwapShareAmountIn"].Error)
	}
}

func testExitSwapShareAmountInTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapShareAmountIn(func(ctx sdk.Context, ex types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]) error {
			testHooksState["testExitSwapShareAmountInTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		exitSwapShareAmountIn(t, ctx, k)
	}
}

func testExitSwapShareAmountInTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testExitSwapShareAmountInTimeout"].Called)
	}
}

// LockTokens tests

func lockTokensTestCoins() []sdk.Coin {
	return []sdk.Coin{
		sdk.NewCoin("gamm/pool/1", sdk.NewInt(42000)),
	}
}

func lockTokens(t *testing.T, ctx sdk.Context, k *Keeper) {
	lockupPeriod := 1 * time.Hour
	testCoins := lockTokensTestCoins()

	_, err := k.TransmitIbcLockTokens(
		ctx,
		owner,
		connectionId,
		timestamp,
		lockupPeriod,
		testCoins,
	)
	require.NoError(t, err)
}

func testLockTokens(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgLockTokens(func(ctx sdk.Context, ex types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) error {
			testHooksState["testLockTokens"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		lockTokens(t, ctx, k)
	}
}

func testLockTokensChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testLockTokens"].Called)
		require.Empty(t, testHooksState["testLockTokens"].Error)
	}
}

func testLockTokensTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgLockTokens(func(ctx sdk.Context, ex types.TimeoutExchange[*lockuptypes.MsgLockTokens]) error {
			testHooksState["testLockTokensTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		lockTokens(t, ctx, k)
	}
}

func testLockTokensTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testLockTokensTimeout"].Called)
	}
}
