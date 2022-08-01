package cli

import (
	"strconv"

	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdRequestWithdrawAll() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "request-withdraw-all [vault-id]",
		Short: "Broadcast message RequestWithdrawAll",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argVaultID := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgRequestWithdrawAll(
				clientCtx.GetFromAddress().String(),
				argVaultID,
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
