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
			// TODO: implement QueryParamsRequest once ehsan implements it
		case contractQuery.QueryGetPoolPositionRequest != nil:
			poolId := contractQuery.QueryGetPoolPositionRequest.PoolId

		case contractQuery.QueryAllPoolPositionsRequest != nil:

		case contractQuery.QueryGetPoolRankingRequest != nil:

		case contractQuery.QueryGetPoolInfoRequest != nil:
			poolId := contractQuery.QueryGetPoolInfoRequest.PoolId

			pool, err := qp.GetPoolInfo(ctx, poolId)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to find pool for id: "+poolId)
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

		case contractQuery.QueryOraclePricesRequest != nil:

		default:
			return nil, wasmvmtypes.UnsupportedRequest{Kind: "unknown osmosis query variant"}
		}
	}
}
