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
		case contractQuery.QueryParamsRequest != nil:
			params := qp.GetParams(ctx)

			res := qoracletypes.QueryParamsResponse{
				Params: params,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal query params response")
			}

			return bz, nil
		case contractQuery.QueryGetPoolPositionRequest != nil:
			poolId := contractQuery.QueryGetPoolPositionRequest.PoolId

			pool, err := qp.GetPoolPosition(ctx, poolId)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "quasar pool position query")
			}

			res := qoracletypes.QueryGetPoolPositionResponse{
				PoolPosition: *pool,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool position query response")
			}

			return bz, nil
		case contractQuery.QueryAllPoolPositionsRequest != nil:
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
		case contractQuery.QueryGetPoolRankingRequest != nil:
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
		case contractQuery.QueryGetPoolInfoRequest != nil:
			poolId := contractQuery.QueryGetPoolInfoRequest.PoolId

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
		case contractQuery.QueryAllPoolInfoRequest != nil:
			pools := qp.GetAllPoolInfo(ctx)

			res := qoracletypes.QueryAllPoolInfoResponse{
				PoolInfo: pools,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool info query response")
			}

			return bz, nil
		case contractQuery.QueryOraclePricesRequest != nil:
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
