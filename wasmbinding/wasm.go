package wasmbinding

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
)

func RegisterCustomPlugins(
	intergammKeeper *intergammkeeper.Keeper,
	bank *bankkeeper.BaseKeeper,
) []wasmkeeper.Option {
	wasmQueryPlugin := NewQueryPlugin(intergammKeeper)

	queryPluginOpt := wasmkeeper.WithQueryPlugins(&wasmkeeper.QueryPlugins{
		Custom: CustomQuerier(wasmQueryPlugin),
	})
	messengerDecoratorOpt := wasmkeeper.WithMessageHandlerDecorator(
		CustomMessageDecorator(intergammKeeper, bank),
	)

	return []wasm.Option{
		queryPluginOpt,
		messengerDecoratorOpt,
	}
}
