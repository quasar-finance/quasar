package main

import (
	"os"

	"github.com/quasarlabs/quasarnode/app"
	appParams "github.com/quasarlabs/quasarnode/app/params"
	"github.com/quasarlabs/quasarnode/cmd/quasarnoded/cmd"
	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	// "github.com/tendermint/spm/cosmoscmd"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
)

func main() {
	rootCmd, _ := cosmoscmd.NewRootCmd(
		appParams.Name,
		appParams.AccountAddressPrefix,
		app.DefaultNodeHome,
		appParams.Name,
		app.ModuleBasics,
		app.New,
		cosmoscmd.AddSubCmd(cmd.TestnetCmd(app.ModuleBasics, banktypes.GenesisBalancesIterator{})),
		// this line is used by starport scaffolding # root/arguments
	)
	if err := svrcmd.Execute(rootCmd, app.DefaultNodeHome); err != nil {
		os.Exit(1)
	}
}
