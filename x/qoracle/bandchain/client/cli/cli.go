package cli

import (
	"github.com/spf13/cobra"
)

// GetQueryCmd returns the query commands for the qoracle bandchain submodule
func GetQueryCmd() *cobra.Command {
	queryCmd := &cobra.Command{
		Use:                        "bandchain",
		Short:                      "qoracle bandchain query subcommands",
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
	}

	queryCmd.AddCommand(
		GetCmdParams(),
		GetCmdState(),
		GetCmdPriceList(),
	)

	return queryCmd
}
