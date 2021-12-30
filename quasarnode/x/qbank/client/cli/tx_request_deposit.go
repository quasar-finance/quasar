package cli

import (
	"strconv"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdRequestDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "request-deposit [risk-profile] [vault-id] [amount] [denom]",
		Short: "Broadcast message requestDeposit",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argRiskProfile := args[0]
			argVaultID := args[1]
			argAmount := args[2]
			argDenom := args[3]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgRequestDeposit(
				clientCtx.GetFromAddress().String(),
				argRiskProfile,
				argVaultID,
				argAmount,
				argDenom,
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
