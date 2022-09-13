package keeper

import (
	"fmt"
	"sort"
	"time"

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

const Year = 365 * 24 * time.Hour

func (k Keeper) TryUpdateOsmosisChainParams(ctx sdk.Context) {
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
			sdk.NewAttribute(types.AttributePacketSequence, fmt.Sprintf("%d", seq)),
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

	state := types.NewOsmosisRequestState(ctx, seq)
	k.setOsmosisParamsRequestState(ctx, state)
	return seq, nil
}

func (k Keeper) TryUpdateOsmosisIncentivizedPools(ctx sdk.Context) {
	state := k.GetOsmosisIncentivizedPoolsRequestState(ctx)
	if state.Pending() {
		k.Logger(ctx).Info("Tried to send a packet to get list of incentivized pools but another request is pending")
		return
	}

	seq, err := k.sendOsmosisIncentivizedPoolsRequest(ctx)
	if err != nil {
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisIncentivizedPoolsRequest,
				sdk.NewAttribute(types.AttributeError, err.Error()),
			))

		k.Logger(ctx).Error("Sending ICQ request to get list of incentivized pools failed", "error", err)
		return
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeOsmosisIncentivizedPoolsRequest,
			sdk.NewAttribute(types.AttributePacketSequence, fmt.Sprintf("%d", seq)),
		))
}

func (k Keeper) sendOsmosisIncentivizedPoolsRequest(ctx sdk.Context) (uint64, error) {
	port := k.GetPort(ctx)
	ibcParams := k.OsmosisParams(ctx).ICQParams

	packetData := types.NewOsmosisIncentivizedPoolsICQPacketData()
	seq, err := k.createOutgoingPacket(ctx, port, ibcParams.AuthorizedChannel, packetData.GetBytes(),
		ibcParams.TimeoutHeight, ibcParams.TimeoutTimestamp)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisRequestState(ctx, seq)
	k.setOsmosisIncentivizedPoolsRequestState(ctx, state)
	return seq, nil
}

func (k Keeper) TryUpdateOsmosisPools(ctx sdk.Context) {
	state := k.GetOsmosisPoolsRequestState(ctx)
	if state.Pending() {
		k.Logger(ctx).Info("Tried to send a packet to get list of pools but another request is pending")
		return
	}

	incentivizedPools := k.GetOsmosisIncentivizedPools(ctx)
	if len(incentivizedPools) == 0 {
		k.Logger(ctx).Info("Tried to send a packet to get list of pools but no incentivized pools are found")
		return
	}

	poolIds := types.UniquePoolIdsFromIncentivizedPools(incentivizedPools)
	seq, err := k.sendOsmosisPoolsRequest(ctx, poolIds)
	if err != nil {
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisPoolsRequest,
				sdk.NewAttribute(types.AttributeError, err.Error()),
			))

		k.Logger(ctx).Error("Sending ICQ request to get list of pools failed", "error", err)
		return
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeOsmosisPoolsRequest,
			sdk.NewAttribute(types.AttributePacketSequence, fmt.Sprintf("%d", seq)),
			sdk.NewAttribute(types.AttributePoolIds, fmt.Sprintf("%v", poolIds)),
		))
}

func (k Keeper) sendOsmosisPoolsRequest(ctx sdk.Context, poolIds []uint64) (uint64, error) {
	port := k.GetPort(ctx)
	ibcParams := k.OsmosisParams(ctx).ICQParams

	packetData := types.NewOsmosisPoolsICQPacketData(poolIds)
	seq, err := k.createOutgoingPacket(ctx, port, ibcParams.AuthorizedChannel, packetData.GetBytes(),
		ibcParams.TimeoutHeight, ibcParams.TimeoutTimestamp)
	if err != nil {
		return 0, err
	}

	state := types.NewOsmosisRequestState(ctx, seq)
	k.setOsmosisPoolsRequestState(ctx, state)
	return seq, nil
}

func (k Keeper) handleOsmosisICQAcknowledgment(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	paramsState := k.GetOsmosisParamsRequestState(ctx)
	incentivizedPoolsState := k.GetOsmosisIncentivizedPoolsRequestState(ctx)
	poolsState := k.GetOsmosisPoolsRequestState(ctx)

	if !ack.Success() {
		// Update the state of osmosis params request if it matches the sequence of packet
		switch packet.Sequence {
		case paramsState.PacketSequence:
			err := k.UpdateOsmosisChainParamsRequestState(ctx, func(state *types.OsmosisRequestState) error {
				state.Failed = true
				return nil
			})
			if err != nil {
				return err
			}
		case incentivizedPoolsState.PacketSequence:
			err := k.UpdateOsmosisIncentivizedPoolsRequestState(ctx, func(state *types.OsmosisRequestState) error {
				state.Failed = true
				return nil
			})
			if err != nil {
				return err
			}
		case poolsState.PacketSequence:
			err := k.UpdateOsmosisPoolsRequestState(ctx, func(state *types.OsmosisRequestState) error {
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
	switch packet.Sequence {
	case paramsState.PacketSequence:
		err := k.UpdateOsmosisChainParamsRequestState(cacheCtx, func(state *types.OsmosisRequestState) error {
			state.Acknowledged = true
			return nil
		})
		if err != nil {
			return err
		}
	case incentivizedPoolsState.PacketSequence:
		err := k.UpdateOsmosisIncentivizedPoolsRequestState(cacheCtx, func(state *types.OsmosisRequestState) error {
			state.Acknowledged = true
			return nil
		})
		if err != nil {
			return err
		}

		// Sends a request to update pools info
		k.TryUpdateOsmosisPools(cacheCtx)
	case poolsState.PacketSequence:
		err := k.UpdateOsmosisPoolsRequestState(cacheCtx, func(state *types.OsmosisRequestState) error {
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

func (k Keeper) UpdateOsmosisChainParamsRequestState(ctx sdk.Context, fn func(state *types.OsmosisRequestState) error) error {
	state := k.GetOsmosisParamsRequestState(ctx)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setOsmosisParamsRequestState(ctx, state)
	return nil
}

func (k Keeper) setOsmosisParamsRequestState(ctx sdk.Context, state types.OsmosisRequestState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyOsmosisParamsRequestState, k.cdc.MustMarshal(&state))
}

// GetOsmosisParamsRequestState returns the state of the osmosis params request
func (k Keeper) GetOsmosisParamsRequestState(ctx sdk.Context) types.OsmosisRequestState {
	store := ctx.KVStore(k.storeKey)
	var state types.OsmosisRequestState
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisParamsRequestState), &state)
	return state
}

func (k Keeper) UpdateOsmosisIncentivizedPoolsRequestState(ctx sdk.Context, fn func(state *types.OsmosisRequestState) error) error {
	state := k.GetOsmosisIncentivizedPoolsRequestState(ctx)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setOsmosisIncentivizedPoolsRequestState(ctx, state)
	return nil
}

func (k Keeper) setOsmosisIncentivizedPoolsRequestState(ctx sdk.Context, state types.OsmosisRequestState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyOsmosisIncentivizedPoolsRequestState, k.cdc.MustMarshal(&state))
}

// GetOsmosisIncentivizedPoolsRequestState returns the state of the osmosis incentivized pools request
func (k Keeper) GetOsmosisIncentivizedPoolsRequestState(ctx sdk.Context) types.OsmosisRequestState {
	store := ctx.KVStore(k.storeKey)
	var state types.OsmosisRequestState
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisIncentivizedPoolsRequestState), &state)
	return state
}

func (k Keeper) UpdateOsmosisPoolsRequestState(ctx sdk.Context, fn func(state *types.OsmosisRequestState) error) error {
	state := k.GetOsmosisPoolsRequestState(ctx)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setOsmosisPoolsRequestState(ctx, state)
	return nil
}

func (k Keeper) setOsmosisPoolsRequestState(ctx sdk.Context, state types.OsmosisRequestState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyOsmosisPoolsRequestState, k.cdc.MustMarshal(&state))
}

// GetOsmosisPoolsRequestState returns the state of the osmosis pools request
func (k Keeper) GetOsmosisPoolsRequestState(ctx sdk.Context) types.OsmosisRequestState {
	store := ctx.KVStore(k.storeKey)
	var state types.OsmosisRequestState
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisPoolsRequestState), &state)
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
	case types.OsmosisQueryDistrInfoPath:
		return k.handleOsmosisDistrInfoResponse(ctx, req, resp)
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "icq response handler for path %s not found", req.Path)
	}
}

func (k Keeper) handleOsmosisEpochsInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp epochtypes.QueryEpochsInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetOsmosisEpochsInfo(ctx, qresp.Epochs)
	return nil
}

func (k Keeper) SetOsmosisEpochsInfo(ctx sdk.Context, epochs []epochtypes.EpochInfo) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisEpochsInfoPrefix)

	for _, epoch := range epochs {
		store.Set([]byte(epoch.Identifier), k.cdc.MustMarshal(&epoch))
	}
}

// GetOsmosisEpochsInfo returns the latest received epochs info from osmosis
func (k Keeper) GetOsmosisEpochsInfo(ctx sdk.Context) []epochtypes.EpochInfo {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisEpochsInfoPrefix)

	iter := store.Iterator(nil, nil)
	defer iter.Close()
	var epochs []epochtypes.EpochInfo
	for ; iter.Valid(); iter.Next() {
		var epoch epochtypes.EpochInfo
		k.cdc.MustUnmarshal(iter.Value(), &epoch)
		epochs = append(epochs, epoch)
	}
	return epochs
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

	metrics, err := k.calculateOsmosisPoolMetrics(ctx, pool)
	if err != nil {
		return sdkerrors.Wrapf(err, "could not calculate pool metrics of pool %d", pool.Id)
	}

	k.SetOsmosisPool(ctx, types.OsmosisPool{
		PoolInfo: pool,
		Metrics:  metrics,
	})
	return nil
}

func (k Keeper) calculateOsmosisPoolMetrics(ctx sdk.Context, pool balancerpool.Pool) (types.OsmosisPoolMetrics, error) {
	tvl, err := k.CalculatePoolTVL(ctx, pool)
	if err != nil {
		return types.OsmosisPoolMetrics{}, sdkerrors.Wrap(err, "could not calculate tvl")
	}
	apy, err := k.CalculatePoolAPY(ctx, pool, tvl)
	if err != nil {
		return types.OsmosisPoolMetrics{}, sdkerrors.Wrap(err, "could not calculate apy")
	}

	return types.OsmosisPoolMetrics{
		APY: apy,
		TVL: tvl,
	}, nil
}

func (k Keeper) SetOsmosisPool(ctx sdk.Context, pool types.OsmosisPool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolPrefix)

	key := sdk.Uint64ToBigEndian(pool.PoolInfo.Id)
	store.Set(key, k.cdc.MustMarshal(&pool))
}

// GetOsmosisPool returns the pool with the given id if exists
func (k Keeper) GetOsmosisPool(ctx sdk.Context, id uint64) (types.OsmosisPool, bool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolPrefix)

	key := sdk.Uint64ToBigEndian(id)
	bz := store.Get(key)
	if bz == nil {
		return types.OsmosisPool{}, false
	}

	var pool types.OsmosisPool
	k.cdc.MustUnmarshal(bz, &pool)
	return pool, true
}

// GetOsmosisPoolsByDenom returns a list of all pools with the desired denom as asset ordered by APY in descending order.
func (k Keeper) GetOsmosisPoolsByDenom(ctx sdk.Context, denom string) []types.OsmosisPool {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolPrefix)

	iter := store.Iterator(nil, nil)
	defer iter.Close()
	var pools types.OsmosisPoolsOrderedByAPY
	for ; iter.Valid(); iter.Next() {
		var pool types.OsmosisPool
		k.cdc.MustUnmarshal(iter.Value(), &pool)

		// Filter out pools with the desired denom as asset
		for _, asset := range pool.PoolInfo.PoolAssets {
			if asset.Token.Denom == denom {
				pools = append(pools, pool)
			}
		}
	}

	// Order by APY in descending order
	sort.Stable(sort.Reverse(pools))
	return pools
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

// GetOsmosisLockableDurations returns the latest received lockable durations from osmosis
func (k Keeper) GetOsmosisLockableDurations(ctx sdk.Context) []time.Duration {
	store := k.getOsmosisStore(ctx)

	var lockableDurations poolincentivestypes.QueryLockableDurationsResponse
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisLockableDurations), &lockableDurations)
	return lockableDurations.LockableDurations
}

func (k Keeper) handleOsmosisMintParamsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryParamsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetOsmosisMintParams(ctx, qresp.Params)
	return nil
}

func (k Keeper) SetOsmosisMintParams(ctx sdk.Context, mintParams minttypes.Params) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyOsmosisMintParams, k.cdc.MustMarshal(&mintParams))
}

// GetOsmosisMintParams returns the latest received mint params from osmosis
func (k Keeper) GetOsmosisMintParams(ctx sdk.Context) minttypes.Params {
	store := k.getOsmosisStore(ctx)

	var mintParams minttypes.Params
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisMintParams), &mintParams)
	return mintParams
}

func (k Keeper) handleOsmosisMintEpochProvisionsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp minttypes.QueryEpochProvisionsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetOsmosisMintEpochProvisions(ctx, qresp.EpochProvisions)
	return nil
}

func (k Keeper) SetOsmosisMintEpochProvisions(ctx sdk.Context, epochProvisions sdk.Dec) {
	store := k.getOsmosisStore(ctx)

	bz, err := epochProvisions.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(types.KeyOsmosisMintEpochProvisions, bz)
}

// GetOsmosisMintEpochProvisions returns the latest received epoch provisions from osmosis
func (k Keeper) GetOsmosisMintEpochProvisions(ctx sdk.Context) sdk.Dec {
	store := k.getOsmosisStore(ctx)
	bz := store.Get(types.KeyOsmosisMintEpochProvisions)

	var epochProvisions sdk.Dec
	err := epochProvisions.Unmarshal(bz)
	if err != nil {
		panic(err)
	}
	return epochProvisions
}

func (k Keeper) handleOsmosisIncentivizedPoolsResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryIncentivizedPoolsResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetOsmosisIncentivizedPools(ctx, qresp.IncentivizedPools)
	return nil
}

func (k Keeper) SetOsmosisIncentivizedPools(ctx sdk.Context, pools []poolincentivestypes.IncentivizedPool) {
	store := k.getOsmosisStore(ctx)
	store.Set(types.KeyOsmosisIncentivizedPools, k.cdc.MustMarshal(&types.IncentivizedPools{IncentivizedPools: pools}))
}

func (k Keeper) GetOsmosisIncentivizedPools(ctx sdk.Context) []poolincentivestypes.IncentivizedPool {
	store := k.getOsmosisStore(ctx)
	var pools types.IncentivizedPools
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisIncentivizedPools), &pools)
	return pools.IncentivizedPools
}

func (k Keeper) handleOsmosisDistrInfoResponse(ctx sdk.Context, req abcitypes.RequestQuery, resp abcitypes.ResponseQuery) error {
	var qresp poolincentivestypes.QueryDistrInfoResponse
	k.cdc.MustUnmarshal(resp.GetValue(), &qresp)

	k.SetOsmosisDistrInfo(ctx, qresp.DistrInfo)
	return nil
}

func (k Keeper) SetOsmosisDistrInfo(ctx sdk.Context, distrInfo poolincentivestypes.DistrInfo) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyOsmosisDistrInfo, k.cdc.MustMarshal(&distrInfo))
}

// GetOsmosisDistrInfo returns the latest received distr info from osmosis
func (k Keeper) GetOsmosisDistrInfo(ctx sdk.Context) poolincentivestypes.DistrInfo {
	store := k.getOsmosisStore(ctx)

	var distrInfo poolincentivestypes.DistrInfo
	k.cdc.MustUnmarshal(store.Get(types.KeyOsmosisDistrInfo), &distrInfo)
	return distrInfo
}

func (k Keeper) handleOsmosisICQTimeout(ctx sdk.Context, packet channeltypes.Packet) error {
	// TODO: Handle timeout
	return nil
}

func (k Keeper) CalculatePoolTVL(ctx sdk.Context, pool balancerpool.Pool) (sdk.Dec, error) {
	tvl := sdk.ZeroDec()
	for _, asset := range pool.PoolAssets {
		price, found := k.GetStablePrice(ctx, asset.Token.Denom)
		if !found {
			return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrStablePriceNotFound, fmt.Sprintf("denom: %s", asset.Token.Denom))
		}

		tvl = tvl.Add(asset.Token.Amount.ToDec().Mul(price))
	}
	return tvl, nil
}

func (k Keeper) CalculatePoolAPY(ctx sdk.Context, pool balancerpool.Pool, poolTVL sdk.Dec) (sdk.Dec, error) {
	distrInfo := k.GetOsmosisDistrInfo(ctx)
	epochProvisions := k.GetOsmosisMintEpochProvisions(ctx)

	mintParams := k.GetOsmosisMintParams(ctx)
	poolIncentivesProportion := mintParams.DistributionProportions.PoolIncentives
	mintDenomPrice, found := k.GetStablePrice(ctx, mintParams.MintDenom)
	if !found {
		return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrStablePriceNotFound, fmt.Sprintf("denom: %s", mintParams.MintDenom))
	}
	mintEpoch, found := k.findOsmosisEpochByIdentifier(ctx, mintParams.EpochIdentifier)
	if !found {
		return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrEpochNotFound, fmt.Sprintf("could not find osmosis mint, epoch identifier: %s", mintParams.EpochIdentifier))
	}

	poolTotalWeight := sdk.ZeroInt()
	for _, incentive := range k.GetOsmosisIncentivizedPools(ctx) {
		if incentive.PoolId == pool.Id {
			gaugeWeight, found := findGaugeWeight(ctx, incentive.GaugeId, distrInfo)
			if !found {
				return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrGaugeWeightNotFound, fmt.Sprintf("gauge id: %d", incentive.GaugeId))
			}
			poolTotalWeight = poolTotalWeight.Add(gaugeWeight)
		}
	}

	// Number of mint epochs occurrence in a year
	annualMintEpochs := Year.Nanoseconds() / mintEpoch.Duration.Nanoseconds()
	annualProvisions := epochProvisions.MulInt64(annualMintEpochs)
	// Annual provisions share to incentivize pools is equal to "annualProvisions * poolIncentivesProportion"
	annualPoolIncentives := annualProvisions.Mul(poolIncentivesProportion)
	// Total annual provision share (including all gauges) of the requested pool in $
	// is equal to "annualPoolIncentives * poolTotalWeight / distrInfo.TotalWeight * mintDenomPrice"
	poolAnnualProvisions := annualPoolIncentives.MulInt(poolTotalWeight).QuoInt(distrInfo.TotalWeight).Mul(mintDenomPrice)
	// APY of the requested pool is equal to "(poolAnnualProvisions / poolTVL) * 100"
	poolAPY := poolAnnualProvisions.Quo(poolTVL).Mul(sdk.NewDec(100))
	return poolAPY, nil
}

// findOsmosisEpochByIdentifier iterates over all osmosis epochs and returns the epoch with given identifier if exists.
func (k Keeper) findOsmosisEpochByIdentifier(ctx sdk.Context, identifier string) (epochtypes.EpochInfo, bool) {
	for _, epoch := range k.GetOsmosisEpochsInfo(ctx) {
		if epoch.Identifier == identifier {
			return epoch, true
		}
	}
	return epochtypes.EpochInfo{}, false
}

// findGaugeWeight iterates over distrInfo.Records and returns the weight of record is it finds and record with given gaugeId.
func findGaugeWeight(ctx sdk.Context, gaugeId uint64, distrInfo poolincentivestypes.DistrInfo) (sdk.Int, bool) {
	for _, record := range distrInfo.Records {
		if record.GaugeId == gaugeId {
			return record.Weight, true
		}
	}
	return sdk.ZeroInt(), false
}
