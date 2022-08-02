package types

import (
	epochtypes "github.com/abag/quasarnode/osmosis/v9/epochs/types"
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
	OsmosisQueryPoolGaugeIdsPath        = "/osmosis.poolincentives.v1beta1.Query/GaugeIds"
	OsmosisQueryDistrInfoPath           = "/osmosis.poolincentives.v1beta1.Query/DistrInfo"
	OsmosisQuerySpotPricePath           = "/osmosis.gamm.v1beta1.Query/SpotPrice"
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

func NewOsmosisParamsRequestState(ctx sdk.Context, seq uint64) OsmosisParamsRequestState {
	return OsmosisParamsRequestState{
		PacketSequence:  seq,
		Acknowledged:    false,
		Failed:          false,
		UpdatedAtHeight: ctx.BlockHeight(),
	}
}

func (state OsmosisParamsRequestState) Pending() bool {
	return state.PacketSequence > 0 && !state.Acknowledged && !state.Failed
}
