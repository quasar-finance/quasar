package keeper

import (
	epochtypes "github.com/abag/quasarnode/osmosis/v7/epochs/types"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icqtypes "github.com/cosmos/ibc-go/v3/modules/apps/icq/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	abcitypes "github.com/tendermint/tendermint/abci/types"
)

func (k Keeper) handleOsmosisICQAcknowledgment(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	if !ack.Success() {
		// TODO: Setting the state of icq request

		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisPacketAcknowledgement,
				sdk.NewAttribute(types.AttributeError, ack.GetError()),
			),
		)
		return nil
	}

	var ackData icqtypes.InterchainQueryPacketAck
	if err := types.ModuleCdc.UnmarshalJSON(ack.GetResult(), &ackData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet acknowledgement data")
	}

	var packetData icqtypes.InterchainQueryPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	cacheCtx, writeCache := ctx.CacheContext()

	for i, req := range packetData.Requests {
		if err := k.handleOsmosisICQResponse(cacheCtx, req, ackData.Responses[i]); err != nil {
			return sdkerrors.Wrapf(err, "could not handle icq response of request %d", i)
		}
	}

	// NOTE: The context returned by CacheContext() creates a new EventManager, so events must be correctly propagated back to the current context
	ctx.EventManager().EmitEvents(cacheCtx.EventManager().Events())
	writeCache()
	return nil
}

func (k Keeper) handleOsmosisICQResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	if resp.IsErr() {
		return sdkerrors.Wrapf(types.ErrFailedICQResponse, "icq response failed with code %d", resp.GetCode())
	}

	switch req.Path {
	case types.OsmosisQueryEpochsInfoPath:
		k.handleOsmosisEpochsInfoResponse(ctx, req, resp)
	case types.OsmosisQueryPoolPath:
	case types.OsmosisQueryLockableDurationsPath:
	case types.OsmosisQueryMintParamsPath:
	case types.OsmosisQueryMintEpochProvisionsPath:
	case types.OsmosisQueryIncentivizedPoolsPath:
	case types.OsmosisQueryPoolGaugeIdsPath:
	case types.OsmosisQueryDistrInfoPath:
	case types.OsmosisQuerySpotPricePath:
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "icq response handler for path %s not found", req.Path)
	}

	// TODO: Remove this once all cases are handled
	return nil
}

func (k Keeper) handleOsmosisEpochsInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp epochtypes.QueryEpochsInfoResponse
	if err := types.ModuleCdc.UnmarshalJSON(resp.GetValue(), &qresp); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal epochs info")
	}

	k.setOsmosisEpochsInfo(ctx, qresp.Epochs)
	return nil
}

func (k Keeper) setOsmosisEpochsInfo(ctx sdk.Context, epochs []epochtypes.EpochInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyOsmosisEpochsInfoPrefix)

	for _, epoch := range epochs {
		store.Set([]byte(epoch.Identifier), k.cdc.MustMarshal(&epoch))
	}
}

func (k Keeper) handleOsmosisICQTimeout(ctx sdk.Context, packet channeltypes.Packet) error {
	// TODO: Handle timeout
	return nil
}
