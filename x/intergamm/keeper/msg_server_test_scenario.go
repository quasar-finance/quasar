package keeper

import (
	"context"
	"errors"
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

var testState map[string]string

func init() {
	testState = make(map[string]string)
}

func (k msgServer) ensureIcaRegistered(ctx sdk.Context, owner string, connectionId string) error {
	var err error
	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return status.Errorf(codes.InvalidArgument, "could not generate port for address: %s", err)
	}

	_, found := k.icaControllerKeeper.GetActiveChannelID(ctx, connectionId, portID)
	if !found {
		err = k.RegisterInterchainAccount(ctx, connectionId, owner)
		if err != nil {
			return err
		}
	}

	return nil
}

func (k msgServer) testRegisterIca(ctx sdk.Context) func(t *testing.T) {
	return func(t *testing.T) {
		var err error
		owner := "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
		connectionId := "connection-0"

		err = k.ensureIcaRegistered(ctx, owner, connectionId)
		require.NoError(t, err)
	}
}

func (k msgServer) testCreatePool(ctx sdk.Context) func(t *testing.T) {
	return func(t *testing.T) {
		// Setup hooks
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
			testState["testCreatePool_hook1"] = "called"
		})
		k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
			testState["testCreatePool_hook2"] = "called"
		})

		owner := "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
		connectionId := "connection-0"
		timestamp := uint64(99999999999999)
		futureGovernor := "168h"

		swapFee, err := sdk.NewDecFromStr("0.01")
		if err != nil {
			panic(err)
		}

		exitFee, err := sdk.NewDecFromStr("0.01")
		if err != nil {
			panic(err)
		}

		poolParams := &gammbalancer.PoolParams{
			SwapFee: swapFee,
			ExitFee: exitFee,
		}

		poolAssets := []gammtypes.PoolAsset{
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uatom", sdk.NewInt(10000)),
			},
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uosmo", sdk.NewInt(10000)),
			},
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uakt", sdk.NewInt(10000)),
			},
		}

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

func (k msgServer) testCreatePoolChecks(ctx sdk.Context) func(t *testing.T) {
	return func(t *testing.T) {
		require.Equal(t, "called", testState["testCreatePool_hook1"])
		require.Equal(t, "called", testState["testCreatePool_hook2"])
	}
}

func (k msgServer) testCreatePoolTimeout(ctx sdk.Context) func(t *testing.T) {
	return func(t *testing.T) {
		// Setup hooks
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) {
			testState["testCreatePoolTimeout_hook1"] = "called"
		})
		k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) {
			testState["testCreatePoolTimeout_hook2"] = "called"
		})

		owner := "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
		connectionId := "connection-0"
		timestamp := uint64(99999999999999)
		futureGovernor := "168h"

		swapFee, err := sdk.NewDecFromStr("0.01")
		if err != nil {
			panic(err)
		}

		exitFee, err := sdk.NewDecFromStr("0.01")
		if err != nil {
			panic(err)
		}

		poolParams := &gammbalancer.PoolParams{
			SwapFee: swapFee,
			ExitFee: exitFee,
		}

		poolAssets := []gammtypes.PoolAsset{
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uatom", sdk.NewInt(10000)),
			},
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uosmo", sdk.NewInt(10000)),
			},
			{
				Weight: sdk.NewInt(100),
				Token:  sdk.NewCoin("uakt", sdk.NewInt(10000)),
			},
		}

		tmpDefaultSendTxRelativeTimeoutTimestamp := DefaultSendTxRelativeTimeoutTimestamp
		DefaultSendTxRelativeTimeoutTimestamp = uint64((time.Duration(200) * time.Millisecond).Nanoseconds())
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

		DefaultSendTxRelativeTimeoutTimestamp = tmpDefaultSendTxRelativeTimeoutTimestamp
	}
}

func (k msgServer) testCreatePoolTimeoutChecks(ctx sdk.Context) func(t *testing.T) {
	return func(t *testing.T) {
		require.Equal(t, "called", testState["testCreatePoolTimeout_hook1"])
		require.Equal(t, "called", testState["testCreatePoolTimeout_hook2"])
	}
}

func (k msgServer) TestScenario(goCtx context.Context, msg *types.MsgTestScenario) (*types.MsgTestScenarioResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.Logger().Info("")
	ctx.Logger().Info("Running test scenario", "scenario", msg.Scenario)
	ctx.Logger().Info("")

	f, ok := map[string]func(*testing.T){
		"registerIca":             k.testRegisterIca(ctx),
		"createPool":              k.testCreatePool(ctx),
		"createPoolChecks":        k.testCreatePoolChecks(ctx),
		"createPoolTimeout":       k.testCreatePoolTimeout(ctx),
		"createPoolTimeoutChecks": k.testCreatePoolTimeoutChecks(ctx),
	}[msg.Scenario]

	if !ok {
		return nil, errors.New("unknown test scenario")
	}

	return runTest(msg.Scenario, f), nil
}
