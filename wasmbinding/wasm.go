package wasmbinding

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
)

func RegisterCustomPlugins(
	intergammKeeper *intergammkeeper.Keeper,
	qoracleKeeper qoraclekeeper.Keeper,
	bank *bankkeeper.BaseKeeper,
	callback *CallbackPlugin,
) []wasmkeeper.Option {
	queryPluginOpt := wasmkeeper.WithQueryPlugins(&wasmkeeper.QueryPlugins{
		Custom: CustomQuerier(qoracleKeeper),
	})
	messengerDecoratorOpt := wasmkeeper.WithMessageHandlerDecorator(
		CustomMessageDecorator(intergammKeeper, bank, callback),
	)

	return []wasm.Option{
		queryPluginOpt,
		messengerDecoratorOpt,
	}
}
