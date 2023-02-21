package types

import (
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/gogo/protobuf/proto"
)

var (
	// CoinRatesMultiplier is the default multiplier used for coin rates oracle requests
	CoinRatesMultiplier uint64 = 1e6
)

type CoinRatesCallDataI interface {
	GetSymbols() []string
	GetMultiplier() uint64
}

type CoinRatesResultI interface {
	GetRates() []uint64
}

var (
	_ CoinRatesCallDataI = (*CoinRatesCallData)(nil)
	_ CoinRatesResultI   = (*CoinRatesResult)(nil)
)

// NewCoinRatesCallData creates a new CoinRatesCallData with symbols and default multiplier.
func NewCoinRatesCallData(symbols []string) CoinRatesCallData {
	return CoinRatesCallData{
		Symbols:    symbols,
		Multiplier: CoinRatesMultiplier,
	}
}

// NewOracleScriptState creates a new OracleScriptState that keeps track of the state of an oracle script request.
func NewOracleScriptState(ctx sdk.Context, clientId string, requestSeq uint64, callData proto.Message) OracleScriptState {
	callDataAny, err := cdctypes.NewAnyWithValue(callData)
	if err != nil {
		panic(err)
	}
	return OracleScriptState{
		ClientId:              clientId,
		CallData:              callDataAny,
		RequestPacketSequence: requestSeq,
		StartedAtHeight:       ctx.BlockHeight(),
		UpdatedAtHeight:       ctx.BlockHeight(),
	}
}

// Pending returns true only if there's no request waiting for result.
func (state OracleScriptState) Pending() bool {
	return state.RequestPacketSequence > 0 && !(state.Result != nil || state.Failed)
}

// SetResult sets the Result of state.
func (state *OracleScriptState) SetResult(result proto.Message) {
	resultAny, err := cdctypes.NewAnyWithValue(result)
	if err != nil {
		panic(err)
	}
	state.Result = resultAny
}

// Fail marks the state as Failed.
func (state *OracleScriptState) Fail() {
	state.Failed = true
}
