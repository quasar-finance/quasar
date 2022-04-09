package cli

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/cosmos/cosmos-sdk/client"

	"github.com/abag/quasarnode/x/osmolpv/types"
)

// GetQueryCmd returns the cli query commands for this module
func GetQueryCmd(queryRoute string) *cobra.Command {
	// Group osmolpv queries under a subcommand
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("Querying commands for the %s module", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(CmdQueryParams())
	cmd.AddCommand(CmdReserveBalance())
	cmd.AddCommand(CmdShowLpPosition())
	cmd.AddCommand(CmdShowEpochLPInfo())
	cmd.AddCommand(CmdShowRewardCollection())
	cmd.AddCommand(CmdShowUserLPInfo())
	cmd.AddCommand(CmdShowLpStat())
	// this line is used by starport scaffolding # 1

	return cmd
}
