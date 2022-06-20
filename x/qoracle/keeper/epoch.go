package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	bandchainParams := k.BandchainParams(ctx)

	switch epochIdentifier {
	case bandchainParams.CoinRatesScriptParams.EpochIdentifier:
		seq, err := k.sendCoinRatesRequest(ctx, types.CoinRatesSymbols, types.CoinRatesMultiplier)
		if err != nil {
			// TODO: Implement a retry mechanism
			ctx.EventManager().EmitEvent(
				sdk.NewEvent(
					types.EventTypeCoinRatesRequest,
					sdk.NewAttribute(types.AttributeError, err.Error()),
					sdk.NewAttribute(types.AttributeEpochIdentifier, epochIdentifier),
					sdk.NewAttribute(types.AttributeEpochNumber, fmt.Sprintf("%d", epochNumber)),
				))
		}
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeCoinRatesRequest,
				sdk.NewAttribute(types.AtributePacketSequence, fmt.Sprintf("%d", seq)),
				sdk.NewAttribute(types.AttributeEpochIdentifier, epochIdentifier),
				sdk.NewAttribute(types.AttributeEpochNumber, fmt.Sprintf("%d", epochNumber)),
			))
	}
}
