package cli

import (
	"fmt"
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdRequestWithdraw() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "request-withdraw [risk-profile] [vault-id] [sdk.coin]",
		Short: "Broadcast message requestWithdraw",
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argRiskProfile := args[0]
			argVaultID := args[1]
			argCoinStr := args[2]

			CoinStr, err := sdk.ParseCoinNormalized(argCoinStr)
			if err != nil {
				return fmt.Errorf("invalid coin string")
			}
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgRequestWithdraw(
				clientCtx.GetFromAddress().String(),
				argRiskProfile,
				argVaultID,
				CoinStr,
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
