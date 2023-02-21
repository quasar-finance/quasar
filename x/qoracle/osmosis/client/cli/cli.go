package cli

import (
	"github.com/spf13/cobra"
)

// GetQueryCmd returns the query commands for the qoracle osmosis submodule
func GetQueryCmd() *cobra.Command {
	queryCmd := &cobra.Command{
		Use:                        "osmosis",
		Short:                      "qoracle osmosis query subcommands",
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
	}

	queryCmd.AddCommand(
		GetCmdParams(),
		GetCmdState(),
		GetCmdChainParams(),
		GetCmdIncentivizedPools(),
		GetCmdPools(),
	)

	return queryCmd
}
