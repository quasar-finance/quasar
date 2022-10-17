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
		case contractQuery.OsmosisPools != nil:
			pools, err := qp.GetAllPools(ctx, contractQuery.OsmosisPools.Pagination)

			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to get all pools")
			}

			res := qoracletypes.QueryOsmosisPoolsResponse{
				Pools: pools,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool query response")
			}

			return bz, nil
		case contractQuery.OsmosisPoolInfo != nil:
			poolId := contractQuery.OsmosisPoolInfo.PoolId

			pool, found := qp.GetPool(ctx, poolId)

			if !found {
				return nil, sdkerrors.ErrKeyNotFound
			}

			res := bindings.OsmosisPoolInfoResponse{
				PoolInfo: pool,
			}

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar pool info query response")
			}

			return bz, nil

		case contractQuery.OraclePrices != nil:
			oraclePrices := qp.GetStablePrices(ctx)

			res := qoracletypes.QueryOraclePricesResponse(oraclePrices)

			bz, err := json.Marshal(res)
			if err != nil {
				return nil, sdkerrors.Wrap(err, "failed to marshal quasar oracle prices query response")
			}

			return bz, nil
		default:
			return nil, wasmvmtypes.UnsupportedRequest{Kind: "unknown custom query variant"}
		}
	}
}
