package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdRegisterICAOnZone() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "register-ica-on-zone [zone-id]",
		Short: "Registers an inter-chain account (ICA) on the given zone",
		Long: `Registers an inter-chain account (ICA) on the given zone.
The owner of the ICA will be the same as the transaction signer
(i.e. it will be determined by the --from flag).`,
		Args: cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argZoneId := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgRegisterICAOnZone(
				clientCtx.GetFromAddress().String(),
				argZoneId,
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
