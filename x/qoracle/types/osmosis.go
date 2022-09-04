package types

import (
	"sort"

	epochtypes "github.com/abag/quasarnode/osmosis/v9/epochs/types"
	gammtypes "github.com/abag/quasarnode/osmosis/v9/gamm/types"
	minttypes "github.com/abag/quasarnode/osmosis/v9/mint/types"
	poolincentivestypes "github.com/abag/quasarnode/osmosis/v9/pool-incentives/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icqtypes "github.com/cosmos/ibc-go/v3/modules/apps/icq/types"
	abcitypes "github.com/tendermint/tendermint/abci/types"
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
	return icqtypes.InterchainQueryPacketData{
		Requests: []abcitypes.RequestQuery{
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
		},
	}
}

func NewOsmosisIncentivizedPoolsICQPacketData() icqtypes.InterchainQueryPacketData {
	return icqtypes.InterchainQueryPacketData{
		Requests: []abcitypes.RequestQuery{
			{
				Path: OsmosisQueryIncentivizedPoolsPath,
				Data: ModuleCdc.MustMarshal(&poolincentivestypes.QueryIncentivizedPoolsRequest{}),
			},
		},
	}
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

	return icqtypes.InterchainQueryPacketData{
		Requests: reqs,
	}
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

type OsmosisPoolsOrderedByAPY []OsmosisPool

var ـ sort.Interface = (OsmosisPoolsOrderedByAPY)(nil)

func (ops OsmosisPoolsOrderedByAPY) Len() int {
	return len(ops)
}

func (ops OsmosisPoolsOrderedByAPY) Less(i, j int) bool {
	return ops[i].Metrics.APY.LT(ops[j].Metrics.APY)
}

func (ops OsmosisPoolsOrderedByAPY) Swap(i, j int) {
	ops[i], ops[j] = ops[j], ops[i]
}
