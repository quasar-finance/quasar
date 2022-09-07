package wasmbinding

import (
	"encoding/json"

	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/wasmbinding/bindings"

	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func CustomQuerier(qp *QueryPlugin) func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
	return func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
		var contractQuery bindings.QuasarQuery
		if err := json.Unmarshal(request, &contractQuery); err != nil {
			return nil, sdkerrors.Wrap(err, "osmosis query")
		}

		switch {
		case contractQuery.OsmosisPoolPosition != nil:
			poolId := contractQuery.OsmosisPoolPosition.PoolId

			// pool, err := qp.GetPoolPosition(ctx, poolId)
			// if err != nil {
			// 	return nil, sdkerrors.Wrap(err, "quasar pool position query")
			// }

			res := qoracletypes.QueryGetPoolPositionResponse{
				PoolPosition: qoracletypes.PoolPosition{
					PoolId: poolId,
					Metrics: &qoracletypes.PoolMetrics{
						HighestAPY: "0.1",
						TVL:        "yor mum",
						GaugeAPYs:  []*qoracletypes.GaugeAPY{{GaugeId: 1337, APY: "0.1", Duration: "1"}},
					},
					LastUpdatedTime: 100000,
					Creator:         "quasar1234",
				},
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool position query response")
			}

			return bz, nil
		case contractQuery.OsmosisAllPoolPositions != nil:
			positions := qp.GetAllPoolPosition(ctx)

			// TODO: this type has a pagination, but the implementation of the query is not paginated yet
			// TODO: Should PoolPosition be renamed to PoolPositions?
			res := qoracletypes.QueryAllPoolPositionResponse{
				PoolPosition: positions,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool position query response")
			}

			return bz, nil
		case contractQuery.OsmosisPoolRanking != nil:
			ranking, err := qp.GetPoolRanking(ctx)

			if err != nil {
				return nil, sdkerrors.Wrap(err, "quasar pool ranking query")
			}

			res := qoracletypes.QueryGetPoolRankingResponse{
				PoolRanking: *ranking,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool ranking query response")
			}

			return bz, nil
		case contractQuery.OsmosisPoolInfo != nil:
			poolId := contractQuery.OsmosisPoolInfo.PoolId

			pool, err := qp.GetPoolInfo(ctx, poolId)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "quasar pool info query")
			}

			res := qoracletypes.QueryGetPoolInfoResponse{
				PoolInfo: *pool,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool info query response")
			}

			return bz, nil
		case contractQuery.OsmosisAllPoolInfo != nil:
			pools := qp.GetAllPoolInfo(ctx)

			res := qoracletypes.QueryAllPoolInfoResponse{
				PoolInfo: pools,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool info query response")
			}

			return bz, nil
		case contractQuery.OraclePrices != nil:
			oraclePrices := qp.GetOraclePrices(ctx)

			res := qoracletypes.QueryOraclePricesResponse(oraclePrices)

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar oracle prices query response")
			}

			return bz, nil
		default:
			return nil, wasmvmtypes.UnsupportedRequest{Kind: "unknown osmosis query variant"}
		}
	}
}
