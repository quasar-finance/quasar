package cli

import (
	"fmt"
	"strconv"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	channelutils "github.com/cosmos/ibc-go/v2/modules/core/04-channel/client/utils"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdSendIbcExitPool() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "send-ibc-exit-pool [src-port] [src-channel]",
		Short: "Broadcast message sendIbcExitPool over IBC",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			creator := clientCtx.GetFromAddress().String()
			srcPort := args[0]
			srcChannel := args[1]
			// Get the relative timeout timestamp
			timeoutTimestamp, err := cmd.Flags().GetUint64(flagPacketTimeoutTimestamp)
			if err != nil {
				return err
			}
			consensusState, _, _, err := channelutils.QueryLatestConsensusState(clientCtx, srcPort, srcChannel)
			if err != nil {
				return err
			}
			if timeoutTimestamp != 0 {
				timeoutTimestamp = consensusState.GetTimestamp() + timeoutTimestamp
			}

			poolId, err := cmd.Flags().GetUint64(FlagPoolId)
			if err != nil {
				return err
			}

			shareAmountInStr, err := cmd.Flags().GetString(FlagShareAmountIn)
			if err != nil {
				return err
			}

			shareAmountIn, ok := sdk.NewIntFromString(shareAmountInStr)
			if !ok {
				return fmt.Errorf("invalid share amount in")
			}

			minAmountsOutStrs, err := cmd.Flags().GetStringArray(FlagMinAmountsOut)
			if err != nil {
				return err
			}

			minAmountsOut := sdk.Coins{}
			for i := 0; i < len(minAmountsOutStrs); i++ {
				parsed, err := sdk.ParseCoinNormalized(minAmountsOutStrs[i])
				if err != nil {
					return err
				}
				minAmountsOut = append(minAmountsOut, parsed)
			}

			msg := types.NewMsgSendIbcExitPool(
				creator,
				srcPort,
				srcChannel,
				timeoutTimestamp,
				poolId,
				shareAmountIn,
				minAmountsOut,
			)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	cmd.Flags().AddFlagSet(FlagSetExitPool())
	cmd.Flags().Uint64(flagPacketTimeoutTimestamp, DefaultRelativePacketTimeoutTimestamp, "Packet timeout timestamp in nanoseconds. Default is 10 minutes.")
	flags.AddTxFlagsToCmd(cmd)

	_ = cmd.MarkFlagRequired(FlagPoolId)
	_ = cmd.MarkFlagRequired(FlagShareAmountIn)
	_ = cmd.MarkFlagRequired(FlagMinAmountsOut)

	return cmd
}
