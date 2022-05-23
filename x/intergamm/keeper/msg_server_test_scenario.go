package keeper

import (
	"context"
	"errors"
	"testing"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func (k msgServer) testBasics(t *testing.T) {
	require.True(t, true)
}

func (k msgServer) TestScenario(goCtx context.Context, msg *types.MsgTestScenario) (*types.MsgTestScenarioResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.Logger().Info("")
	ctx.Logger().Info("Running test scenario", "scenario", msg.Scenario)
	ctx.Logger().Info("")

	f, ok := map[string]func(*testing.T){
		"testBasics": k.testBasics,
	}[msg.Scenario]

	if !ok {
		return nil, errors.New("unknown test scenario")
	}

	return runTest(msg.Scenario, f), nil
}
