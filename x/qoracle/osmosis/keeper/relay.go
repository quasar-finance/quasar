package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icqtypes "github.com/cosmos/ibc-go/v5/modules/apps/icq/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/utils"
	abcitypes "github.com/tendermint/tendermint/abci/types"
)

func (k Keeper) sendParamsRequest(ctx sdk.Context) (uint64, error) {
	packetData := types.NewOsmosisParamsICQPacketData()
	packet, err := utils.SendPacket(
		ctx,
		k.clientKeeper,
		k.ics4Wrapper,
		k.channelKeeper,
		k.scopedKeeper,
		k.GetPort(ctx),
		k.GetAuthorizedChannel(ctx),
		packetData.GetBytes(),
		k.GetPacketTimeoutHeight(ctx),
		k.GetPacketTimeoutTimestamp(ctx),
	)
	EmitOsmosisRequestEvent(ctx, "chain_params", packet, err)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisRequestState(ctx, packet.GetSequence())
	k.setRequestState(ctx, types.KeyParamsRequestState, state)

	return packet.GetSequence(), nil
}

func (k Keeper) UpdateRequestState(
	ctx sdk.Context,
	key []byte,
	fn func(state *types.OsmosisRequestState) error,
) error {
	state := k.GetRequestState(ctx, key)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setRequestState(ctx, key, state)
	return nil
}

func (k Keeper) setRequestState(ctx sdk.Context, key []byte, state types.OsmosisRequestState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(key, k.cdc.MustMarshal(&state))
}

// GetRequestState returns the state of the osmosis request given its key
func (k Keeper) GetRequestState(ctx sdk.Context, key []byte) types.OsmosisRequestState {
	store := ctx.KVStore(k.storeKey)
	var state types.OsmosisRequestState
	k.cdc.MustUnmarshal(store.Get(key), &state)
	return state
}

func (k Keeper) TryUpdateIncentivizedPools(ctx sdk.Context) {
	// Do not start a new procedure if module is disabled
	if !k.IsEnabled(ctx) {
		k.Logger(ctx).Info("module is disabled, skipping IncentivizedPools update")
		return
	}

	// Do not start a new procedure if there's another one pending
	state := k.GetRequestState(ctx, types.KeyIncentivizedPoolsRequestState)
	if state.Pending() {
		k.Logger(ctx).Info("tried to update IncentivizedPools but another request is pending")
		return
	}

	_, err := k.sendIncentivizedPoolsRequest(ctx)
	if err != nil {
		k.Logger(ctx).Error("could not send IncentivizedPools request to osmosis", "error", err)
		return
	}
}

func (k Keeper) sendIncentivizedPoolsRequest(ctx sdk.Context) (uint64, error) {
	packetData := types.NewOsmosisIncentivizedPoolsICQPacketData()
	packet, err := utils.SendPacket(
		ctx,
		k.clientKeeper,
		k.ics4Wrapper,
		k.channelKeeper,
		k.scopedKeeper,
		k.GetPort(ctx),
		k.GetAuthorizedChannel(ctx),
		packetData.GetBytes(),
		k.GetPacketTimeoutHeight(ctx),
		k.GetPacketTimeoutTimestamp(ctx),
	)
	EmitOsmosisRequestEvent(ctx, "incentivized_pools", packet, err)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisRequestState(ctx, packet.GetSequence())
	k.setRequestState(ctx, types.KeyIncentivizedPoolsRequestState, state)

	return packet.GetSequence(), nil
}

func (k Keeper) TryUpdatePools(ctx sdk.Context) {
	// Do not start a new procedure if module is disabled
	if !k.IsEnabled(ctx) {
		k.Logger(ctx).Info("module is disabled, skipping Pools update")
		return
	}

	// Do not start a new procedure if there's another one pending
	state := k.GetRequestState(ctx, types.KeyPoolsRequestState)
	if state.Pending() {
		k.Logger(ctx).Info("tried to update Pools but another request is pending")
		return
	}

	incentivizedPools := k.GetIncentivizedPools(ctx)
	if len(incentivizedPools) == 0 {
		k.Logger(ctx).Info("empty IncentivizedPools list, skipping Pools update")
		// Remove all the pools in store, because we have to reflect exactly what we receive from osmosis
		k.removeAllPools(ctx)
		return
	}

	poolIds := types.UniquePoolIdsFromIncentivizedPools(incentivizedPools)
	_, err := k.sendPoolsRequest(ctx, poolIds)
	if err != nil {
		k.Logger(ctx).Error("could not send Pools request to osmosis", "error", err)
		return
	}
}

func (k Keeper) sendPoolsRequest(ctx sdk.Context, poolIds []uint64) (uint64, error) {
	packetData := types.NewOsmosisPoolsICQPacketData(poolIds)
	packet, err := utils.SendPacket(
		ctx,
		k.clientKeeper,
		k.ics4Wrapper,
		k.channelKeeper,
		k.scopedKeeper,
		k.GetPort(ctx),
		k.GetAuthorizedChannel(ctx),
		packetData.GetBytes(),
		k.GetPacketTimeoutHeight(ctx),
		k.GetPacketTimeoutTimestamp(ctx),
	)
	EmitOsmosisRequestEvent(ctx, "pools", packet, err)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisRequestState(ctx, packet.GetSequence())
	k.setRequestState(ctx, types.KeyPoolsRequestState, state)

	return packet.GetSequence(), nil
}

func (k Keeper) OnAcknowledgementPacket(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	paramsState := k.GetRequestState(ctx, types.KeyParamsRequestState)
	incentivizedPoolsState := k.GetRequestState(ctx, types.KeyIncentivizedPoolsRequestState)
	poolsState := k.GetRequestState(ctx, types.KeyPoolsRequestState)

	if !ack.Success() {
		// Update the state of osmosis params request if it matches the sequence of packet
		switch packet.GetSequence() {
		case paramsState.GetPacketSequence():
			err := k.UpdateRequestState(ctx, types.KeyParamsRequestState, func(state *types.OsmosisRequestState) error {
				state.Fail()
				return nil
			})
			if err != nil {
				return err
			}
		case incentivizedPoolsState.GetPacketSequence():
			err := k.UpdateRequestState(ctx, types.KeyIncentivizedPoolsRequestState, func(state *types.OsmosisRequestState) error {
				state.Fail()
				return nil
			})
			if err != nil {
				return err
			}
		case poolsState.GetPacketSequence():
			err := k.UpdateRequestState(ctx, types.KeyPoolsRequestState, func(state *types.OsmosisRequestState) error {
				state.Fail()
				return nil
			})
			if err != nil {
				return err
			}
		}

		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisPacketAcknowledgement,
				sdk.NewAttribute(types.AttributeKeyError, ack.GetError()),
			),
		)
		return nil
	}

	var ackData icqtypes.InterchainQueryPacketAck
	if err := types.ModuleCdc.UnmarshalJSON(ack.GetResult(), &ackData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal icq packet acknowledgement data")
	}
	resps, err := icqtypes.DeserializeCosmosResponse(ackData.Data)
	if err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal icq acknowledgement data to cosmos response")
	}

	var packetData icqtypes.InterchainQueryPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal icq packet data")
	}
	reqs, err := icqtypes.DeserializeCosmosQuery(packetData.Data)
	if err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal icq packet data to cosmos query")
	}

	cacheCtx, writeCache := ctx.CacheContext()

	for i, req := range reqs {
		if err := k.handleOsmosisICQResponse(cacheCtx, req, resps[i]); err != nil {
			return sdkerrors.Wrapf(err, "could not handle icq response of request %d", i)
		}
	}

	// Update the state of osmosis params request if it matches the sequence of packet
	switch packet.Sequence {
	case paramsState.PacketSequence:
		err := k.UpdateRequestState(cacheCtx, types.KeyParamsRequestState, func(state *types.OsmosisRequestState) error {
			state.Success()
			return nil
		})
		if err != nil {
			return err
		}
	case incentivizedPoolsState.PacketSequence:
		err := k.UpdateRequestState(cacheCtx, types.KeyIncentivizedPoolsRequestState, func(state *types.OsmosisRequestState) error {
			state.Success()
			return nil
		})
		if err != nil {
			return err
		}

		// Sends a request to update pools info
		// TODO: Move this to EndBlock handler as it consumes gas from relayer
		k.TryUpdatePools(cacheCtx)
	case poolsState.PacketSequence:
		k.SetPoolsUpdatedAt(cacheCtx, cacheCtx.BlockTime())

		err := k.UpdateRequestState(cacheCtx, types.KeyPoolsRequestState, func(state *types.OsmosisRequestState) error {
			state.Success()
			return nil
		})
		if err != nil {
			return err
		}

		k.qoracleKeeper.NotifyPoolsUpdate(ctx)
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
		return k.handleOsmosisEpochsInfoResponse(ctx, req, resp)
	case types.OsmosisQueryPoolPath:
		return k.handleOsmosisPoolResponse(ctx, req, resp)
	case types.OsmosisQueryLockableDurationsPath:
		return k.handleOsmosisLockableDurationsResponse(ctx, req, resp)
	case types.OsmosisQueryMintParamsPath:
		return k.handleOsmosisMintParamsResponse(ctx, req, resp)
	case types.OsmosisQueryMintEpochProvisionsPath:
		return k.handleOsmosisMintEpochProvisionsResponse(ctx, req, resp)
	case types.OsmosisQueryIncentivizedPoolsPath:
		return k.handleOsmosisIncentivizedPoolsResponse(ctx, req, resp)
	case types.OsmosisQueryDistrInfoPath:
		return k.handleOsmosisDistrInfoResponse(ctx, req, resp)
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "icq response handler for path %s not found", req.Path)
	}
}

func (k Keeper) handleOsmosisEpochsInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp epochtypes.QueryEpochsInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetEpochsInfo(ctx, qresp.Epochs)
	return nil
}

func (k Keeper) handleOsmosisPoolResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp gammtypes.QueryPoolResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	var pool balancerpool.Pool
	err := pool.Unmarshal(qresp.GetPool().GetValue())
	if err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal pool")
	}

	k.SetPool(ctx, pool)
	return nil
}

func (k Keeper) handleOsmosisLockableDurationsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryLockableDurationsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetLockableDurations(ctx, qresp)
	return nil
}

func (k Keeper) handleOsmosisMintParamsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryParamsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetMintParams(ctx, qresp.Params)
	return nil
}

func (k Keeper) handleOsmosisMintEpochProvisionsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryEpochProvisionsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetMintEpochProvisions(ctx, qresp.EpochProvisions)
	return nil
}

func (k Keeper) handleOsmosisIncentivizedPoolsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryIncentivizedPoolsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetIncentivizedPools(ctx, qresp.IncentivizedPools)
	return nil
}

func (k Keeper) handleOsmosisDistrInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryDistrInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetDistrInfo(ctx, qresp.DistrInfo)
	return nil
}

func (k Keeper) OnTimeoutPacket(ctx sdk.Context, packet channeltypes.Packet) error {
	// TODO: Handle timeout
	return nil
}
