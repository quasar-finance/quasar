package cli

import (
	"strconv"
	"time"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	transfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdForwardIbcTransfer() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "forward-ibc-transfer",
		Short: "Broadcast message forwardIbcTransfer [connection-id] [transfer-port] [transfer-channel] [token] [fwd-transfer-port] [fwd-transfer-channel] [intermediate-receiver] [receiver]",
		Args:  cobra.ExactArgs(8),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			connectionId := args[0]
			transferPort := args[1]
			transferChannel := args[2]
			fwdTransferPort := args[4]
			fwdTransferChannel := args[5]
			intermediateReceiver := args[6]
			receiver := args[7]

			coin, err := sdk.ParseCoinNormalized(args[3])
			if err != nil {
				return err
			}

			timeoutHeightStr, err := cmd.Flags().GetString(flagPacketTimeoutHeight)
			if err != nil {
				return err
			}
			timeoutHeight, err := clienttypes.ParseHeight(timeoutHeightStr)
			if err != nil {
				return err
			}

			timeoutTimestamp, err := cmd.Flags().GetUint64(flagPacketTimeoutTimestamp)
			if err != nil {
				return err
			}
			timeoutTimestamp = uint64(time.Now().Add(time.Duration(timeoutTimestamp)).UnixNano())

			msg := types.NewMsgForwardIbcTransfer(
				clientCtx.GetFromAddress().String(),
				connectionId,
				uint64(time.Now().Add(time.Hour).UnixNano()),
				transferPort,
				transferChannel,
				coin,
				fwdTransferPort,
				fwdTransferChannel,
				intermediateReceiver,
				receiver,
				timeoutHeight,
				timeoutTimestamp,
			)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	cmd.Flags().String(flagPacketTimeoutHeight, transfertypes.DefaultRelativePacketTimeoutHeight, "Packet timeout block height. The timeout is disabled when set to 0-0.")
	cmd.Flags().Uint64(flagPacketTimeoutTimestamp, transfertypes.DefaultRelativePacketTimeoutTimestamp, "Packet timeout timestamp in nanoseconds from now. Default is 10 minutes. The timeout is disabled when set to 0.")
	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
