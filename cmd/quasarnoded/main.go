package main

import (
	"os"

	"github.com/cosmos/cosmos-sdk/server"
	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"

	app "github.com/quasarlabs/quasarnode/app"
	"github.com/quasarlabs/quasarnode/cmd/quasarnoded/cmd"
)

var ENV_PREXIX = "QUASARNODED"

// "Looks good to me. Ready for launch. LFG" -@valeyo
func main() {
	rootCmd, _ := cmd.NewRootCmd()

	if err := svrcmd.Execute(rootCmd, ENV_PREXIX, app.DefaultNodeHome); err != nil {
		switch e := err.(type) {
		case server.ErrorCode:
			os.Exit(e.Code)

		default:
			os.Exit(1)
		}
	}
}
