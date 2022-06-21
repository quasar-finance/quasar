//go:build !prod

package keeper

import (
	"testing"
	"time"

	"github.com/abag/quasarnode/x/intergamm/types"
	gammtypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm"
	gammbalancer "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
	lockuptypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/lockup"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	transfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

const (
	owner              = "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
	osmosisAddress     = "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"
	cosmosAddress      = "cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs"
	connectionId       = "connection-0"
	transferPortId     = "transfer"
	transferChannelId  = "channel-0"
	fwdTransferPort    = "transfer"
	fwdTransferChannel = "channel-1"
	poolId             = uint64(1)
	timestamp          = uint64(99999999999999)
)

var (
	transferTimeoutTimestamp = uint64((time.Duration(10) * time.Minute).Nanoseconds())
	transferTimeoutHeight    = clienttypes.NewHeight(0, 0)
)

type HookState struct {
	Called       bool
	Error        string
	LastSequence uint64
}

var testHooksState map[string]HookState

func scenario(f func(sdk.Context, *Keeper) func(*testing.T)) func(string, sdk.Context, *Keeper) *types.MsgTestScenarioResponse {
	return func(name string, ctx sdk.Context, k *Keeper) *types.MsgTestScenarioResponse {
		return runTest(name, f(ctx, k))
	}
}

func init() {
	testHooksState = make(map[string]HookState)

	scenarios["registerIca"] = scenario(testRegisterIca)
	scenarios["createPool"] = scenario(testCreatePool)
	scenarios["createPoolChecks"] = scenario(testCreatePoolChecks)
	scenarios["createPoolTimeout"] = scenario(testCreatePoolTimeout)
	scenarios["createPoolTimeoutChecks"] = scenario(testCreatePoolTimeoutChecks)
	scenarios["joinPool"] = scenario(testJoinPool)
	scenarios["joinPoolChecks"] = scenario(testJoinPoolChecks)
	scenarios["joinPoolTimeout"] = scenario(testJoinPoolTimeout)
	scenarios["joinPoolTimeoutChecks"] = scenario(testJoinPoolTimeoutChecks)
	scenarios["exitPool"] = scenario(testExitPool)
	scenarios["exitPoolChecks"] = scenario(testExitPoolChecks)
	scenarios["exitPoolTimeout"] = scenario(testExitPoolTimeout)
	scenarios["exitPoolTimeoutChecks"] = scenario(testExitPoolTimeoutChecks)
	scenarios["joinSwapExternAmountIn"] = scenario(testJoinSwapExternAmountIn)
	scenarios["joinSwapExternAmountInChecks"] = scenario(testJoinSwapExternAmountInChecks)
	scenarios["joinSwapExternAmountInTimeout"] = scenario(testJoinSwapExternAmountInTimeout)
	scenarios["joinSwapExternAmountInTimeoutChecks"] = scenario(testJoinSwapExternAmountInTimeoutChecks)
	scenarios["exitSwapExternAmountOut"] = scenario(testExitSwapExternAmountOut)
	scenarios["exitSwapExternAmountOutChecks"] = scenario(testExitSwapExternAmountOutChecks)
	scenarios["exitSwapExternAmountOutTimeout"] = scenario(testExitSwapExternAmountOutTimeout)
	scenarios["exitSwapExternAmountOutTimeoutChecks"] = scenario(testExitSwapExternAmountOutTimeoutChecks)
	scenarios["joinSwapShareAmountOut"] = scenario(testJoinSwapShareAmountOut)
	scenarios["joinSwapShareAmountOutChecks"] = scenario(testJoinSwapShareAmountOutChecks)
	scenarios["joinSwapShareAmountOutTimeout"] = scenario(testJoinSwapShareAmountOutTimeout)
	scenarios["joinSwapShareAmountOutTimeoutChecks"] = scenario(testJoinSwapShareAmountOutTimeoutChecks)
	scenarios["exitSwapShareAmountIn"] = scenario(testExitSwapShareAmountIn)
	scenarios["exitSwapShareAmountInChecks"] = scenario(testExitSwapShareAmountInChecks)
	scenarios["exitSwapShareAmountInTimeout"] = scenario(testExitSwapShareAmountInTimeout)
	scenarios["exitSwapShareAmountInTimeoutChecks"] = scenario(testExitSwapShareAmountInTimeoutChecks)
	scenarios["lockTokens"] = scenario(testLockTokens)
	scenarios["lockTokensChecks"] = scenario(testLockTokensChecks)
	scenarios["lockTokensTimeout"] = scenario(testLockTokensTimeout)
	scenarios["lockTokensTimeoutChecks"] = scenario(testLockTokensTimeoutChecks)
	scenarios["beginUnlocking"] = scenario(testBeginUnlocking)
	scenarios["beginUnlockingChecks"] = scenario(testBeginUnlockingChecks)
	scenarios["beginUnlockingTimeout"] = scenario(testBeginUnlockingTimeout)
	scenarios["beginUnlockingTimeoutChecks"] = scenario(testBeginUnlockingTimeoutChecks)
	scenarios["transferIbcTokens"] = scenario(testTransferIbcTokens)
	scenarios["transferIbcTokensChecks"] = scenario(testTransferIbcTokensChecks)
	scenarios["forwardTransferIbcTokens"] = scenario(testForwardTransferIbcTokens)
	scenarios["forwardTransferIbcTokensChecks"] = scenario(testForwardTransferIbcTokensChecks)
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
		k.Hooks.IbcTransfer.ClearAckHooks()
		k.Hooks.IbcTransfer.ClearTimeoutHooks()
		k.Hooks.Osmosis.ClearAckHooks()
		k.Hooks.Osmosis.ClearTimeoutHooks()
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

func createTestPoolAssets() []gammbalancer.PoolAsset {
	return []gammbalancer.PoolAsset{
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

// BeginUnlocking tests

func beginUnlocking(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoins := lockTokensTestCoins()

	_, err := k.TransmitIbcBeginUnlocking(
		ctx,
		owner,
		connectionId,
		timestamp,
		1,
		testCoins,
	)
	require.NoError(t, err)
}

func testBeginUnlocking(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksAckMsgBeginUnlocking(func(ctx sdk.Context, ex types.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse]) error {
			testHooksState["testBeginUnlocking"] = HookState{
				Called: true,
				Error:  ex.Error,
			}
			return nil
		})

		lockTokens(t, ctx, k)
	}
}

func testBeginUnlockingChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testBeginUnlocking"].Called)
		require.Empty(t, testHooksState["testBeginUnlocking"].Error)
	}
}

func testBeginUnlockingTimeout(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.Osmosis.AddHooksTimeoutMsgBeginUnlocking(func(ctx sdk.Context, ex types.TimeoutExchange[*lockuptypes.MsgBeginUnlocking]) error {
			testHooksState["testBeginUnlockingTimeout"] = HookState{
				Called: true,
			}
			return nil
		})

		defer swapTimeout()()

		lockTokens(t, ctx, k)
	}
}

func testBeginUnlockingTimeoutChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testBeginUnlockingTimeout"].Called)
	}
}

var transferIbcTokensLastSequence uint64 = 0

func transferIbcTokensTestCoin() sdk.Coin {
	return sdk.NewCoin("qsr", sdk.NewInt(10))
}

func transferIbcTokens(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoin := transferIbcTokensTestCoin()
	sender, err := sdk.AccAddressFromBech32(owner)
	require.NoError(t, err)

	seq, err := k.TransferIbcTokens(
		ctx,
		transferPortId,
		transferChannelId,
		testCoin,
		sender,
		osmosisAddress,
		transferTimeoutHeight,
		uint64(time.Now().UnixNano())+transferTimeoutTimestamp,
	)
	require.NoError(t, err)
	require.NotZero(t, seq)

	transferIbcTokensLastSequence = seq
}

func testTransferIbcTokens(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.IbcTransfer.AddHooksAckIbcTransfer(func(ctx sdk.Context, ex types.AckExchange[*transfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error {
			testHooksState["testTransferIbcTokens"] = HookState{
				Called:       true,
				Error:        ex.Error,
				LastSequence: ex.Sequence,
			}
			return nil
		})

		transferIbcTokens(t, ctx, k)
	}
}

func testTransferIbcTokensChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testTransferIbcTokens"].Called)
		require.Empty(t, testHooksState["testTransferIbcTokens"].Error)
		require.Equal(t, transferIbcTokensLastSequence, testHooksState["testTransferIbcTokens"].LastSequence)
	}
}

func forwardTransferIbcTokens(t *testing.T, ctx sdk.Context, k *Keeper) {
	testCoin := transferIbcTokensTestCoin()
	sender, err := sdk.AccAddressFromBech32(owner)
	require.NoError(t, err)

	seq, err := k.ForwardTransferIbcTokens(
		ctx,
		transferPortId, transferChannelId,
		testCoin,
		sender,
		fwdTransferPort, fwdTransferChannel,
		cosmosAddress,
		osmosisAddress,
		transferTimeoutHeight,
		uint64(time.Now().UnixNano())+transferTimeoutTimestamp,
	)
	require.NoError(t, err)
	require.NotZero(t, seq)

	transferIbcTokensLastSequence = seq
}

func testForwardTransferIbcTokens(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		k.Hooks.IbcTransfer.AddHooksAckIbcTransfer(func(ctx sdk.Context, ex types.AckExchange[*transfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error {
			testHooksState["testForwardTransferIbcTokens"] = HookState{
				Called:       true,
				Error:        ex.Error,
				LastSequence: ex.Sequence,
			}
			return nil
		})

		forwardTransferIbcTokens(t, ctx, k)
	}
}

func testForwardTransferIbcTokensChecks(ctx sdk.Context, k *Keeper) func(t *testing.T) {
	return func(t *testing.T) {
		require.True(t, testHooksState["testForwardTransferIbcTokens"].Called)
		require.Empty(t, testHooksState["testForwardTransferIbcTokens"].Error)
		require.Equal(t, transferIbcTokensLastSequence, testHooksState["testForwardTransferIbcTokens"].LastSequence)
	}
}
