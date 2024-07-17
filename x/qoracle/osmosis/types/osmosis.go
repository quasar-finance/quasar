package types

import (
	abcitypes "github.com/cometbft/cometbft/abci/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icqtypes "github.com/cosmos/ibc-apps/modules/async-icq/v8/types"

	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
)

const (
	OsmosisQueryEpochsInfoPath          = "/osmosis.epochs.v1beta1.Query/EpochInfos"
	OsmosisQueryPoolPath                = "/osmosis.gamm.v1beta1.Query/Pool"
	OsmosisQueryLockableDurationsPath   = "/osmosis.poolincentives.v1beta1.Query/LockableDurations"
	OsmosisQueryMintParamsPath          = "/osmosis.mint.v1beta1.Query/Params"
	OsmosisQueryMintEpochProvisionsPath = "/osmosis.mint.v1beta1.Query/EpochProvisions"
	OsmosisQueryIncentivizedPoolsPath   = "/osmosis.poolincentives.v1beta1.Query/IncentivizedPools"
	OsmosisQueryDistrInfoPath           = "/osmosis.poolincentives.v1beta1.Query/DistrInfo"
)

func NewOsmosisParamsICQPacketData() icqtypes.InterchainQueryPacketData {
	data, err := icqtypes.SerializeCosmosQuery([]abcitypes.RequestQuery{
		{
			Path: OsmosisQueryEpochsInfoPath,
			Data: ModuleCdc.MustMarshal(&epochtypes.QueryEpochsInfoRequest{}),
		},
		{
			Path: OsmosisQueryLockableDurationsPath,
			Data: ModuleCdc.MustMarshal(&poolincentivestypes.QueryLockableDurationsRequest{}),
		},
		{
			Path: OsmosisQueryMintParamsPath,
			Data: ModuleCdc.MustMarshal(&minttypes.QueryParamsRequest{}),
		},
		{
			Path: OsmosisQueryMintEpochProvisionsPath,
			Data: ModuleCdc.MustMarshal(&minttypes.QueryEpochProvisionsRequest{}),
		},
		{
			Path: OsmosisQueryDistrInfoPath,
			Data: ModuleCdc.MustMarshal(&poolincentivestypes.QueryDistrInfoRequest{}),
		},
	})
	if err != nil {
		panic(err)
	}

	return icqtypes.InterchainQueryPacketData{Data: data}
}

func NewOsmosisIncentivizedPoolsICQPacketData() icqtypes.InterchainQueryPacketData {
	data, err := icqtypes.SerializeCosmosQuery([]abcitypes.RequestQuery{
		{
			Path: OsmosisQueryIncentivizedPoolsPath,
			Data: ModuleCdc.MustMarshal(&poolincentivestypes.QueryIncentivizedPoolsRequest{}),
		},
	})
	if err != nil {
		panic(err)
	}

	return icqtypes.InterchainQueryPacketData{Data: data}
}

func NewOsmosisPoolsICQPacketData(poolIds []uint64) icqtypes.InterchainQueryPacketData {
	reqs := make([]abcitypes.RequestQuery, len(poolIds))
	for i, poolId := range poolIds {
		reqs[i] = abcitypes.RequestQuery{
			Path: OsmosisQueryPoolPath,
			Data: ModuleCdc.MustMarshal(&gammtypes.QueryPoolRequest{
				PoolId: poolId,
			}),
		}
	}

	data, err := icqtypes.SerializeCosmosQuery(reqs)
	if err != nil {
		panic(err)
	}
	return icqtypes.InterchainQueryPacketData{Data: data}
}

// UniquePoolIdsFromIncentivizedPools returns the unique pool ids from an array of incentivized pools.
func UniquePoolIdsFromIncentivizedPools(incentivizedPools []poolincentivestypes.IncentivizedPool) []uint64 {
	poolIds := make([]uint64, 0, len(incentivizedPools))
	for _, pool := range incentivizedPools {
		skip := false
		for _, id := range poolIds {
			if id == pool.PoolId {
				skip = true
				break
			}
		}
		if skip {
			continue
		}

		poolIds = append(poolIds, pool.PoolId)
	}
	return poolIds
}

func NewOsmosisRequestState(ctx sdk.Context, seq uint64) OsmosisRequestState {
	return OsmosisRequestState{
		PacketSequence:  seq,
		Acknowledged:    false,
		Failed:          false,
		UpdatedAtHeight: ctx.BlockHeight(),
	}
}

func (state OsmosisRequestState) Pending() bool {
	return state.PacketSequence > 0 && !state.Acknowledged && !state.Failed
}

func (state *OsmosisRequestState) Success() {
	state.Acknowledged = true
}

func (state *OsmosisRequestState) Fail() {
	state.Failed = true
}
