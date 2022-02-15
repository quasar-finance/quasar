package cli

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
)

func CmdCreatePoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool-position [poolID] [apy] [tvl] [last-updated-time]",
		Short: "Create PoolPosition",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			argPoolID, err := cast.ToUint64E(args[0])
			if err != nil {
				return err
			}

			argAPY, err := cast.ToUint64E(args[1])
			if err != nil {
				return err
			}
			argTVL, err := cast.ToUint64E(args[2])
			if err != nil {
				return err
			}
			argLastUpdatedTime, err := cast.ToUint64E(args[3])
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgCreatePoolPosition(argPoolID, clientCtx.GetFromAddress().String(), argAPY, argTVL, argLastUpdatedTime)
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
		Use:   "update-pool-position [poolID][apy] [tvl] [last-updated-time]",
		Short: "Update PoolPosition",
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			argPoolID, err := cast.ToUint64E(args[0])
			if err != nil {
				return err
			}

			argAPY, err := cast.ToUint64E(args[1])
			if err != nil {
				return err
			}
			argTVL, err := cast.ToUint64E(args[2])
			if err != nil {
				return err
			}
			argLastUpdatedTime, err := cast.ToUint64E(args[3])
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgUpdatePoolPosition(clientCtx.GetFromAddress().String(), argPoolID, argAPY, argTVL, argLastUpdatedTime)
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
		Use:   "delete-pool-position",
		Short: "Delete PoolPosition",
		Args:  cobra.ExactArgs(1),
		// Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}
			argPoolID, err := cast.ToUint64E(args[0])
			if err != nil {
				return err
			}
			msg := types.NewMsgDeletePoolPosition(clientCtx.GetFromAddress().String(), argPoolID)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
