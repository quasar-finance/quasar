package cli

import (
	"io/ioutil"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/pkg/errors"
	"github.com/spf13/cobra"
	pflag "github.com/spf13/pflag"
)

const (
	// The connection end identifier on the controller chain
	FlagConnectionID = "connection-id"
)

// common flagsets to add to various functions
var (
	fsConnectionID = pflag.NewFlagSet("", pflag.ContinueOnError)
)

func init() {
	fsConnectionID.String(FlagConnectionID, "", "Connection ID")
}

func CmdSubmitRaw() *cobra.Command {
	cmd := &cobra.Command{
		Use:  "submit [path/to/sdk_msg.json]",
		Args: cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			cdc := codec.NewProtoCodec(clientCtx.InterfaceRegistry)

			var txMsg sdk.Msg
			if err := cdc.UnmarshalInterfaceJSON([]byte(args[0]), &txMsg); err != nil {

				// check for file path if JSON input is not provided
				contents, err := ioutil.ReadFile(args[0])
				if err != nil {
					return errors.Wrap(err, "neither JSON input nor path to .json file for sdk msg were provided")
				}

				if err := cdc.UnmarshalInterfaceJSON(contents, &txMsg); err != nil {
					return errors.Wrap(err, "error unmarshalling sdk msg file")
				}
			}

			connectionId, err := fsConnectionID.GetString(FlagConnectionID)
			if err != nil {
				return err
			}

			msg, err := types.NewMsgSubmitTx(txMsg, connectionId, clientCtx.GetFromAddress().String())
			if err != nil {
				return err
			}

			if err := msg.ValidateBasic(); err != nil {
				return err
			}

			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	cmd.Flags().AddFlagSet(fsConnectionID)
	// fsConnectionID.String(FlagConnectionID, "", "Connection ID")
	_ = cmd.MarkFlagRequired(FlagConnectionID)

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
