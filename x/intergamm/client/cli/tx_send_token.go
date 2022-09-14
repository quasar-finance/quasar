package cli

import (
	"errors"
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdSendToken() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "send-token [to-zone-id] [to-address] [coin]",
		Short: "Send funds from one account to another on different zone.",
		Long:  `Send funds from one account to another on different zone.`,
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			argToZoneId := args[0]
			if argToZoneId == "" {
				msg := "error: to-zone-id must be non-empty"
				_ = clientCtx.PrintString(msg)
				return errors.New(msg)
			}
			argToAddress := args[1]
			if argToAddress == "" {
				msg := "error: to-address must be non-empty"
				_ = clientCtx.PrintString(msg)
				return errors.New(msg)
			}
			argCoin, err := sdk.ParseCoinNormalized(args[2])
			if err != nil {
				return err
			}
			if !(argCoin.IsValid() && argCoin.IsPositive()) {
				msg := "error: amount must be valid and positive"
				_ = clientCtx.PrintString(msg)
				return errors.New(msg)
			}

			msg := types.NewMsgSendToken(
				clientCtx.GetFromAddress().String(),
				argToZoneId,
				argToAddress,
				argCoin,
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
