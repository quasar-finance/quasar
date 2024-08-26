package main

import (
	"os"

	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"

	"github.com/quasar-finance/quasar/app"
	appparams "github.com/quasar-finance/quasar/app/params"
	"github.com/quasar-finance/quasar/cmd/quasard/cmd"
)

// "Looks good to me. Ready for launch. LFG" -@valeyo
func main() {
	appparams.SetAddressPrefixes()
	rootCmd := cmd.NewRootCmd()

	if err := svrcmd.Execute(rootCmd, "", app.DefaultNodeHome); err != nil {
		os.Exit(1)
	}
}
