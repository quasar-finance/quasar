package cli

import (
	"time"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

func CmdCreatePoolInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool-info [pool-id] [poolFile.json]",
		Short: "Create a new PoolInfo",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]

			// Get value arguments
			argInfo, err := parseBalancerPoolFile(args[1])
			if err != nil {
				return err
			}
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgCreatePoolInfo(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				argInfo,
				argLastUpdatedTime,
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

func CmdUpdatePoolInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update-pool-info [pool-id] [poolFile.json]",
		Short: "Update a PoolInfo",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]

			// Get value arguments
			argInfo, err := parseBalancerPoolFile(args[1])
			if err != nil {
				return err
			}
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgUpdatePoolInfo(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				argInfo,
				argLastUpdatedTime,
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

func CmdDeletePoolInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delete-pool-info [pool-id]",
		Short: "Delete a PoolInfo",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			indexPoolId := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgDeletePoolInfo(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
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
