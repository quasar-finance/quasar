package cli

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
)

const (
	listSeparator = ","
)

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("%s transactions subcommands", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(CmdCreatePoolPosition())
	cmd.AddCommand(CmdUpdatePoolPosition())
	cmd.AddCommand(CmdDeletePoolPosition())
	cmd.AddCommand(CmdCreatePoolRanking())
	cmd.AddCommand(CmdUpdatePoolRanking())
	cmd.AddCommand(CmdDeletePoolRanking())
	cmd.AddCommand(CmdCreatePoolSpotPrice())
	cmd.AddCommand(CmdUpdatePoolSpotPrice())
	cmd.AddCommand(CmdDeletePoolSpotPrice())
	cmd.AddCommand(CmdCreatePoolInfo())
	cmd.AddCommand(CmdUpdatePoolInfo())
	cmd.AddCommand(CmdDeletePoolInfo())
	cmd.AddCommand(CmdStablePrice())
	cmd.AddCommand(CmdUpdateOsmosisChainParams())
	// this line is used by starport scaffolding # 1

	return cmd
}
