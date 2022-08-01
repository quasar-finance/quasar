package cli

import (
	"strconv"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdStablePrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "stable-price [denom] [price]",
		Short: "Broadcast message StablePrice",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argDenom := args[0]
			argPrice := args[1]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgStablePrice(
				clientCtx.GetFromAddress().String(),
				argDenom,
				argPrice,
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
