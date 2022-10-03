package main

import (
	"os"

	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	ignitecmd "github.com/ignite/modules/cmd"
	"github.com/quasarlabs/quasarnode/app"
	appParams "github.com/quasarlabs/quasarnode/app/params"
	"github.com/quasarlabs/quasarnode/cmd/quasarnoded/cmd"
)

func main() {
	rootCmd, _ := ignitecmd.NewRootCmd(
		appParams.Name,
		appParams.AccountAddressPrefix,
		app.DefaultNodeHome,
		appParams.Name,
		app.ModuleBasics,
		app.New,
		ignitecmd.AddSubCmd(cmd.TestnetCmd(app.ModuleBasics, banktypes.GenesisBalancesIterator{})),
		// this line is used by starport scaffolding # root/arguments
	)
	if err := svrcmd.Execute(rootCmd, "", app.DefaultNodeHome); err != nil {
		os.Exit(1)
	}
}
