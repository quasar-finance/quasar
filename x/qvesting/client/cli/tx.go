package cli

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"

	"github.com/quasarlabs/quasarnode/x/qvesting/types"
)

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("%s transactions subcommands", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(CmdCreateVestingAccount())

	return cmd
}

func CmdCreateVestingAccount() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-vesting-account [to-address] [amount] [start-time] [end-time]",
		Short: "Create a new vesting account funded with an allocation of tokens.",
		Long: `Create a new vesting account funded with an allocation of tokens. 
The account can be a continuous vesting account. The start_time and end_time must be 
provided as a UNIX epoch timestamp.`,
		Args: cobra.ExactArgs(4),
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
