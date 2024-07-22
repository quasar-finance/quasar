package main

import (
	"os"

	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"

	"github.com/quasarlabs/quasarnode/app"
	"github.com/quasarlabs/quasarnode/cmd/quasarnoded/cmd"
)

// "Looks good to me. Ready for launch. LFG" -@valeyo
func main() {
	rootCmd, _ := cmd.NewRootCmd()

	if err := svrcmd.Execute(rootCmd, "", app.DefaultNodeHome); err != nil {
		{
			os.Exit(1)
		}
	}
}
