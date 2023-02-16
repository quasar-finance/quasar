package wasmbinding

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
)

func RegisterCustomPlugins(
	// intergammKeeper *intergammkeeper.Keeper,
	qoracleKeeper *qoraclekeeper.Keeper,
	bank *bankkeeper.BaseKeeper,
	callback *CallbackPlugin,
) []wasmkeeper.Option {
	// wasmQueryPlugin := NewQueryPlugin(intergammKeeper, qoracleKeeper)
	wasmQueryPlugin := NewQueryPlugin(qoracleKeeper)

	queryPluginOpt := wasmkeeper.WithQueryPlugins(&wasmkeeper.QueryPlugins{
		Custom: CustomQuerier(wasmQueryPlugin),
	})
	messengerDecoratorOpt := wasmkeeper.WithMessageHandlerDecorator(
		//		CustomMessageDecorator(intergammKeeper, bank, callback),
		CustomMessageDecorator(bank, callback),
	)

	return []wasm.Option{
		queryPluginOpt,
		messengerDecoratorOpt,
	}
}
