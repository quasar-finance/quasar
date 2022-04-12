package cli

import (
	"strconv"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdInterchainAccountFromAddress() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "interchain-account-from-address [connection-id] [owner]",
		Short: "Query interchainAccountFromAddress",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryInterchainAccountFromAddressRequest{
				Owner:        args[1],
				ConnectionId: args[0],
			}
			res, err := queryClient.InterchainAccountFromAddress(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
