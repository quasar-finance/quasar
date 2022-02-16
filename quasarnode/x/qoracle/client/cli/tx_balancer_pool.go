package cli

import (
	"strconv"
)

var _ = strconv.Itoa(0)

/*
func CmdBalancerPool() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "balancer-pool [address] [uid]",
		Short: "Broadcast message BalancerPool",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argAddress := args[0]
			argUid, err := cast.ToUint64E(args[1])
			if err != nil {
				return err
			}

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgBalancerPool(
				clientCtx.GetFromAddress().String(),
				argAddress,
				argUid,
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
*/
