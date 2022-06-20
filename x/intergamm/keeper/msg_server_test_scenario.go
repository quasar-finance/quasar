package keeper

import (
	"context"
	"errors"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

var scenarios map[string]func(string, sdk.Context, *Keeper) *types.MsgTestScenarioResponse

func init() {
	scenarios = make(map[string]func(string, sdk.Context, *Keeper) *types.MsgTestScenarioResponse)
}

func (ms msgServer) TestScenario(goCtx context.Context, msg *types.MsgTestScenario) (*types.MsgTestScenarioResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.Logger().Info("Running test scenario", "scenario", msg.Scenario)

	f, ok := scenarios[msg.Scenario]

	if !ok {
		return nil, errors.New("unknown test scenario")
	}

	return f(msg.Scenario, ctx, ms.k), nil
}
