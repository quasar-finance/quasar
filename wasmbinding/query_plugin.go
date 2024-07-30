package wasmbinding

import (
	"encoding/json"

	errorsmod "cosmossdk.io/errors"
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/wasmbinding/bindings"
)

func CustomQuerier() func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
	return func(ctx sdk.Context, request json.RawMessage) ([]byte, error) {
		var contractQuery bindings.QuasarQuery
		if err := json.Unmarshal(request, &contractQuery); err != nil {
			return nil, errorsmod.Wrap(err, "osmosis query")
		}

		switch {
		default:
			return nil, wasmvmtypes.UnsupportedRequest{Kind: "unknown custom query variant"}
		}
	}
}
