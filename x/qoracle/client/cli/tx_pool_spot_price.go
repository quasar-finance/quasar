package cli

import (
	"time"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

func CmdCreatePoolSpotPrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool-spot-price [pool-id] [denom-in] [denom-out] [price]",
		Short: "Create a new PoolSpotPrice",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]
			indexDenomIn := args[1]
			indexDenomOut := args[2]

			// Get value arguments
			argPrice := args[3]
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgCreatePoolSpotPrice(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				indexDenomIn,
				indexDenomOut,
				argPrice,
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

func CmdUpdatePoolSpotPrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update-pool-spot-price [pool-id] [denom-in] [denom-out] [price]",
		Short: "Update a PoolSpotPrice",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			// Get indexes
			indexPoolId := args[0]
			indexDenomIn := args[1]
			indexDenomOut := args[2]

			// Get value arguments
			argPrice := args[3]
			argLastUpdatedTime, err := cast.ToUint64E(time.Now().Unix())
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgUpdatePoolSpotPrice(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				indexDenomIn,
				indexDenomOut,
				argPrice,
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

func CmdDeletePoolSpotPrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delete-pool-spot-price [pool-id] [denom-in] [denom-out]",
		Short: "Delete a PoolSpotPrice",
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			indexPoolId := args[0]
			indexDenomIn := args[1]
			indexDenomOut := args[2]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgDeletePoolSpotPrice(
				clientCtx.GetFromAddress().String(),
				indexPoolId,
				indexDenomIn,
				indexDenomOut,
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
