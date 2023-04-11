package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/vestingcustom/types"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

// Transaction command flags
const (
	FlagDelayed = "delayed"
)

func CmdCreateVestingAccount() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-vesting-account [to-address] [amount] [start-time] [end-time]",
		Short: "Broadcast message CreateVestingAccount",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argToAddress, err := sdk.AccAddressFromBech32(args[0])
			if err != nil {
				return err
			}

			argAmount, err := sdk.ParseCoinsNormalized(args[1])
			if err != nil {
				return err
			}

			argStartTime, err := cast.ToInt64E(args[2])
			if err != nil {
				return err
			}

			argEndTime, err := cast.ToInt64E(args[3])
			if err != nil {
				return err
			}

			delayed, _ := cmd.Flags().GetBool(FlagDelayed)

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgCreateVestingAccount(
				clientCtx.GetFromAddress().String(),
				argToAddress,
				argAmount,
				argStartTime,
				argEndTime,
				delayed,
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
