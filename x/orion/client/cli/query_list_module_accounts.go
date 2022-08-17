package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdListModuleAccounts() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list-module-accounts",
		Short: "Query list_module_accounts",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryListModuleAccountsRequest{}

			res, err := queryClient.ListModuleAccounts(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
