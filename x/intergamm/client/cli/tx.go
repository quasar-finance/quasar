package cli

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
)

const (
	flagPacketTimeoutTimestamp = "packet-timeout-timestamp"
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

	cmd.AddCommand(CmdRegisterAccount())
	cmd.AddCommand(CmdCreatePool())
	cmd.AddCommand(CmdJoinPool())
	cmd.AddCommand(CmdExitPool())
	cmd.AddCommand(CmdIbcTransfer())
	cmd.AddCommand(CmdForwardIbcTransfer())
	cmd.AddCommand(CmdTransferIbcTokens())
	cmd.AddCommand(CmdForwardTransferIbcTokens())
	// this line is used by starport scaffolding # 1

	return cmd
}
