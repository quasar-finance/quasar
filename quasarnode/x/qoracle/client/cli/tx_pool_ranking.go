package cli

import (
	"strings"
	"time"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

func CmdCreatePoolRanking() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool-ranking [pool-ids-sorted-by-apy] [pool-ids-sorted-by-tvl]",
		Short: "Create PoolRanking",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argPoolIdsSortedByAPY := strings.Split(args[0], listSeparator)
			argPoolIdsSortedByTVL := strings.Split(args[1], listSeparator)
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgCreatePoolRanking(clientCtx.GetFromAddress().String(), argPoolIdsSortedByAPY, argPoolIdsSortedByTVL, argLastUpdatedTime)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}

func CmdUpdatePoolRanking() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update-pool-ranking [pool-ids-sorted-by-apy] [pool-ids-sorted-by-tvl]",
		Short: "Update PoolRanking",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argPoolIdsSortedByAPY := strings.Split(args[0], listSeparator)
			argPoolIdsSortedByTVL := strings.Split(args[1], listSeparator)
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgUpdatePoolRanking(clientCtx.GetFromAddress().String(), argPoolIdsSortedByAPY, argPoolIdsSortedByTVL, argLastUpdatedTime)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}

func CmdDeletePoolRanking() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delete-pool-ranking",
		Short: "Delete PoolRanking",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgDeletePoolRanking(clientCtx.GetFromAddress().String())
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
