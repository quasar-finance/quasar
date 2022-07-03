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

// NewCoinRatesCallDataFromDecCoins creates a new CoinRatesCallData with coins symbols and default multiplier.
func NewCoinRatesCallDataFromDecCoins(coins sdk.DecCoins) CoinRatesCallData {
	symbols := make([]string, len(coins))
	for i, coin := range coins {
		symbols[i] = coin.GetDenom()
	}

	return CoinRatesCallData{
		Symbols:    symbols,
		Multiplier: CoinRatesMultiplier,
	}
}

func NewOracleScriptState(ctx sdk.Context, requestSeq uint64, callData proto.Message) OracleScriptState {
	callDataAny, err := cdctypes.NewAnyWithValue(callData)
	if err != nil {
		panic(err)
	}
	return OracleScriptState{
		CallData:              callDataAny,
		RequestPacketSequence: requestSeq,
		UpdatedAtHeight:       ctx.BlockHeight(),
	}
}

// Pending returns true only if there's no request waiting for result.
func (state OracleScriptState) Pending() bool {
	return state.RequestPacketSequence > 0 && !(state.Result != nil || state.Failed)
}

func (state *OracleScriptState) SetResult(result proto.Message) {
	resultAny, err := cdctypes.NewAnyWithValue(result)
	if err != nil {
		panic(err)
	}
	state.Result = resultAny
}
