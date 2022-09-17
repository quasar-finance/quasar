package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdTransmitICATransfer() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "transmit-ica-transfer [ica-zone-id] [to-address] [amount]",
		Short: "Transmits a message to the ICA zone that will cause a fund transfer from there to quasar",
		Long: `Transmits a message to the ICA zone that will cause a fund transfer from there to quasar.
The destination address are specified with to-address, respectively.
The owner of the source ICA is the transaction signer
(i.e. it will be determined by the --from flag).
If no such ICA exists, an error will be returned, therefore the ICA must be registered first.`,
		Args: cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argIcaZoneId := args[0]
			argToAddress := args[1]
			argAmount, err := sdk.ParseCoinNormalized(args[2])
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgTransmitICATransfer(
				clientCtx.GetFromAddress().String(),
				argIcaZoneId,
				argToAddress,
				argAmount,
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
