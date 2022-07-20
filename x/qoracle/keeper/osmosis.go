package keeper

import (
	"encoding/binary"
	"fmt"

	epochtypes "github.com/abag/quasarnode/osmosis/v9/epochs/types"
	balancerpool "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/osmosis/v9/gamm/types"
	minttypes "github.com/abag/quasarnode/osmosis/v9/mint/types"
	poolincentivestypes "github.com/abag/quasarnode/osmosis/v9/pool-incentives/types"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icqtypes "github.com/cosmos/ibc-go/v3/modules/apps/icq/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	abcitypes "github.com/tendermint/tendermint/abci/types"
)

func (k Keeper) TryUpdateOsmosisParams(ctx sdk.Context) {
	state := k.GetOsmosisParamsRequestState(ctx)
	if state.Pending() {
		k.Logger(ctx).Info("Tried to send a packet to update osmosis params but another request is pending")
		return
	}

	seq, err := k.sendOsmosisParamsRequest(ctx)
	if err != nil {
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisParamsRequest,
				sdk.NewAttribute(types.AttributeError, err.Error()),
			))

		k.Logger(ctx).Error("Sending ICQ request to update osmosis params failed", "error", err)
		return
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeOsmosisParamsRequest,
			sdk.NewAttribute(types.AtributePacketSequence, fmt.Sprintf("%d", seq)),
		))
}

func (k Keeper) sendOsmosisParamsRequest(ctx sdk.Context) (uint64, error) {
	port := k.GetPort(ctx)
	ibcParams := k.OsmosisParams(ctx).ICQParams

	packetData := types.NewOsmosisParamsICQPacketData()
	seq, err := k.createOutgoingPacket(ctx, port, ibcParams.AuthorizedChannel, packetData.GetBytes(),
		ibcParams.TimeoutHeight, ibcParams.TimeoutTimestamp)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisParamsRequestState(ctx, seq)
	k.setOsmosisParamsRequestState(ctx, state)
	return seq, nil
}

func (k Keeper) handleOsmosisICQAcknowledgment(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	state := k.GetOsmosisParamsRequestState(ctx)

	if !ack.Success() {
		// Update the state of osmosis params request if it matches the sequence of packet
		if packet.Sequence == state.PacketSequence {
			err := k.updateOsmosisParamsRequestState(ctx, func(state *types.OsmosisParamsRequestState) error {
				state.Failed = true
				return nil
			})
			if err != nil {
				return err
			}
		}

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

	// Update the state of osmosis params request if it matches the sequence of packet
	if packet.Sequence == state.PacketSequence {
		err := k.updateOsmosisParamsRequestState(cacheCtx, func(state *types.OsmosisParamsRequestState) error {
			state.Acknowledged = true
			return nil
		})
		if err != nil {
			return err
		}
	}

	// NOTE: The context returned by CacheContext() creates a new EventManager, so events must be correctly propagated back to the current context
	ctx.EventManager().EmitEvents(cacheCtx.EventManager().Events())
	writeCache()
	return nil
}

func (k Keeper) updateOsmosisParamsRequestState(ctx sdk.Context, fn func(state *types.OsmosisParamsRequestState) error) error {
	state := k.GetOsmosisParamsRequestState(ctx)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setOsmosisParamsRequestState(ctx, state)
	return nil
}

func (k Keeper) setOsmosisParamsRequestState(ctx sdk.Context, state types.OsmosisParamsRequestState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyOsmosisParamsRequestState, k.cdc.MustMarshal(&state))
}

// GetOsmosisParamsRequestState returns the state of the osmosis params request
func (k Keeper) GetOsmosisParamsRequestState(ctx sdk.Context) types.OsmosisParamsRequestState {
	store := ctx.KVStore(k.storeKey)
	var state types.OsmosisParamsRequestState
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisParamsRequestState), &state)
	return state
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
	case types.OsmosisQueryPoolGaugeIdsPath:
		return k.handleOsmosisPoolGaugeIdsResponse(ctx, req, resp)
	case types.OsmosisQueryDistrInfoPath:
		return k.handleOsmosisDistrInfoResponse(ctx, req, resp)
	case types.OsmosisQuerySpotPricePath:
		return k.handleOsmosisSpotPriceResponse(ctx, req, resp)
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "icq response handler for path %s not found", req.Path)
	}
}

func (k Keeper) handleOsmosisEpochsInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp epochtypes.QueryEpochsInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisEpochsInfo(ctx, qresp.Epochs)
	return nil
}

func (k Keeper) setOsmosisEpochsInfo(ctx sdk.Context, epochs []epochtypes.EpochInfo) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisEpochsInfoPrefix)

	for _, epoch := range epochs {
		store.Set([]byte(epoch.Identifier), k.cdc.MustMarshal(&epoch))
	}
}

// getOsmosisStore returns the prefix store for osmosis incoming data
func (k Keeper) getOsmosisStore(ctx sdk.Context) prefix.Store {
	return prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyOsmosisPrefix)
}

func (k Keeper) handleOsmosisPoolResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp gammtypes.QueryPoolResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	var pool balancerpool.Pool
	err := pool.Unmarshal(qresp.GetPool().GetValue())
	if err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal pool")
	}

	k.setOsmosisPool(ctx, pool)
	return nil
}

func (k Keeper) setOsmosisPool(ctx sdk.Context, pool balancerpool.Pool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolPrefix)

	key := make([]byte, 8)
	binary.BigEndian.PutUint64(key, pool.Id) // TODO: Ensure that big endian is the correct bit order

	store.Set(key, k.cdc.MustMarshal(&pool))
}

func (k Keeper) handleOsmosisLockableDurationsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryLockableDurationsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisLockableDurations(ctx, qresp)
	return nil
}

func (k Keeper) setOsmosisLockableDurations(ctx sdk.Context, lockableDurations poolincentivestypes.QueryLockableDurationsResponse) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyOsmosisLockableDurations, k.cdc.MustMarshal(&lockableDurations))
}

func (k Keeper) handleOsmosisMintParamsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryParamsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisMintParams(ctx, qresp.Params)
	return nil
}

func (k Keeper) setOsmosisMintParams(ctx sdk.Context, mintParams minttypes.Params) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyOsmosisMintParams, k.cdc.MustMarshal(&mintParams))
}

func (k Keeper) handleOsmosisMintEpochProvisionsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryEpochProvisionsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisMintEpochProvisions(ctx, qresp.EpochProvisions)
	return nil
}

func (k Keeper) setOsmosisMintEpochProvisions(ctx sdk.Context, epochProvisions sdk.Dec) {
	store := k.getOsmosisStore(ctx)

	bz, err := epochProvisions.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(types.KeyOsmosisMintEpochProvisions, bz)
}

func (k Keeper) handleOsmosisIncentivizedPoolsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryIncentivizedPoolsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisIncentivizedPools(ctx, qresp.IncentivizedPools)
	return nil
}

func (k Keeper) setOsmosisIncentivizedPools(ctx sdk.Context, pools []poolincentivestypes.IncentivizedPool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisIncentivizedPoolsPrefix)

	for _, pool := range pools {
		key := make([]byte, 8)
		binary.BigEndian.PutUint64(key, pool.PoolId) // TODO: Ensure that big endian is the correct bit order

		store.Set(key, k.cdc.MustMarshal(&pool))
	}
}

func (k Keeper) handleOsmosisPoolGaugeIdsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qreq poolincentivestypes.QueryGaugeIdsRequest
	k.cdc.MustUnmarshal(req.GetData(), &qreq)

	var qresp poolincentivestypes.QueryGaugeIdsResponse_GaugeIdWithDuration
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisPoolGaugeIds(ctx, qreq.PoolId, qresp)
	return nil
}

func (k Keeper) setOsmosisPoolGaugeIds(ctx sdk.Context, poolId uint64, gaugeIdWithDuration poolincentivestypes.QueryGaugeIdsResponse_GaugeIdWithDuration) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolGaugeIdsPrefix)

	key := make([]byte, 8)
	binary.BigEndian.PutUint64(key, poolId) // TODO: Ensure that big endian is the correct bit order

	store.Set(key, k.cdc.MustMarshal(&gaugeIdWithDuration))
}

func (k Keeper) handleOsmosisDistrInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryDistrInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.setOsmosisDistrInfo(ctx, qresp.DistrInfo)
	return nil
}

func (k Keeper) setOsmosisDistrInfo(ctx sdk.Context, distrInfo poolincentivestypes.DistrInfo) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyOsmosisDistrInfo, k.cdc.MustMarshal(&distrInfo))
}

func (k Keeper) handleOsmosisSpotPriceResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qreq gammtypes.QuerySpotPriceRequest
	k.cdc.MustUnmarshal(req.GetData(), &qreq)

	var qresp gammtypes.QuerySpotPriceResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	spotPrice := sdk.MustNewDecFromStr(qresp.SpotPrice)
	k.setOsmosisSpotPrice(ctx, qreq.PoolId, qreq.BaseAssetDenom, qreq.QuoteAssetDenom, spotPrice)
	return nil
}

func (k Keeper) setOsmosisSpotPrice(ctx sdk.Context, poolId uint64, base, quote string, spotPrice sdk.Dec) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisSpotPricePrefix)

	bz, err := spotPrice.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(types.CreateOsmosisSpotPriceKey(poolId, base, quote), bz)
}

func (k Keeper) handleOsmosisICQTimeout(ctx sdk.Context, packet channeltypes.Packet) error {
	// TODO: Handle timeout
	return nil
}
