package main

import (
	"os"

	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"
	"github.com/quasar-finance/quasar/app"
	"github.com/quasar-finance/quasar/cmd/quasard/cmd"
)

// "Looks good to me. Ready for launch. LFG" -@valeyo
func main() {
	rootCmd, _ := cmd.NewRootCmd()

	if err := svrcmd.Execute(rootCmd, "OSMOSISD", app.DefaultNodeHome); err != nil {
		os.Exit(1)
	}
}
