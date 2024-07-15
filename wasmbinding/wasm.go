package wasmbinding

import (
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"

	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
)

func RegisterCustomPlugins(
	//	intergammKeeper *intergammkeeper.Keeper,
	qoracleKeeper qoraclekeeper.Keeper,
	bank *bankkeeper.BaseKeeper,
	callback *CallbackPlugin,
) []wasmkeeper.Option {
	queryPluginOpt := wasmkeeper.WithQueryPlugins(&wasmkeeper.QueryPlugins{
		Custom: CustomQuerier(qoracleKeeper),
	})
	messengerDecoratorOpt := wasmkeeper.WithMessageHandlerDecorator(
		CustomMessageDecorator(bank, callback),
	)

	return []wasmkeeper.Option{
		queryPluginOpt,
		messengerDecoratorOpt,
	}
}
