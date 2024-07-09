package wasmbinding

import (
	errorsmod "cosmossdk.io/errors"
	"encoding/json"

	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/wasmbinding/bindings"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
)

func CustomQuerier(qk qoraclekeeper.Keeper) func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
	return func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
		var contractQuery bindings.QuasarQuery
		if err := json.Unmarshal(request, &contractQuery); err != nil {
			return nil, errorsmod.Wrap(err, "osmosis query")
		}

		switch {
		case contractQuery.PoolsRankedByAPY != nil:
			pools := qk.GetPoolsRankedByAPY(ctx, contractQuery.PoolsRankedByAPY.Denom)

			bz, err := json.Marshal(pools)
			if err != nil {
				return nil, errorsmod.Wrap(err, "failed to marshal quasar qoracle pools")
			}
			return bz, nil
		case contractQuery.Pool != nil:
			pool, found := qk.GetPool(ctx, contractQuery.Pool.Id)
			if !found {
				return nil, errorsmod.Wrap(sdkerrors.ErrInvalidRequest, "pool not found")
			}

			bz, err := json.Marshal(pool)
			if err != nil {
				return nil, errorsmod.Wrap(err, "failed to marshal quasar pool")
			}
			return bz, nil
		/*
			case contractQuery.TokenPrice != nil:
				price, err := qk.GetDenomPrice(ctx, contractQuery.TokenPrice.Denom)
				if err != nil {
					return nil, errorsmod.Wrap(err, "failed to get token price")
				}

				bz, err := price.MarshalJSON()
				if err != nil {
					return nil, errorsmod.Wrap(err, "failed to marshal quasar token price")
				}
				return bz, nil

		*/
		default:
			return nil, wasmvmtypes.UnsupportedRequest{Kind: "unknown custom query variant"}
		}
	}
}
