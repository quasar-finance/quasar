package cli

import (
	"encoding/json"
	"time"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

func CmdCreatePoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool-position [pool-id] [metrics]",
		Short: "Create a new PoolPosition",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]

			// Get value arguments
			argMetrics := new(types.PoolMetrics)
			err = json.Unmarshal([]byte(args[1]), argMetrics)
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

			msg := types.NewMsgCreatePoolPosition(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				argMetrics,
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

func CmdUpdatePoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update-pool-position [pool-id] [metrics]",
		Short: "Update a PoolPosition",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]

			// Get value arguments
			argMetrics := new(types.PoolMetrics)
			err = json.Unmarshal([]byte(args[1]), argMetrics)
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

			msg := types.NewMsgUpdatePoolPosition(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				argMetrics,
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

func CmdDeletePoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delete-pool-position [pool-id]",
		Short: "Delete a PoolPosition",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			indexPoolId := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgDeletePoolPosition(
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
