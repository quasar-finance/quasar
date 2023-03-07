package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        "osmosis",
		Short:                      "qoracle osmosis tx subcommands",
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	// cmd.AddCommand(CmdUpdateOsmosisChainParams())
	// this line is used by starport scaffolding # 1

	return cmd
}

/*
func CmdUpdateOsmosisChainParams() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update-osmosis-chain-params",
		Short: "Broadcast message UpdateOsmosisChainParams",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := qosmotypes.NewMsgUpdateChainParams(
				clientCtx.GetFromAddress().String(),
			)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
*/
