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

var cmds []*cobra.Command

func addCommand(cmd *cobra.Command) {
	cmds = append(cmds, cmd)
}

func init() {
	addCommand(CmdRegisterAccount())
	addCommand(CmdCreatePool())
	addCommand(CmdJoinPool())
	addCommand(CmdExitPool())
	addCommand(CmdIbcTransfer())
	addCommand(CmdForwardIbcTransfer())
	addCommand(CmdTransferIbcTokens())
	addCommand(CmdForwardTransferIbcTokens())
}

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("%s transactions subcommands", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}
	cmd.AddCommand(cmds...)
	// this line is used by starport scaffolding # 1

	return cmd
}
