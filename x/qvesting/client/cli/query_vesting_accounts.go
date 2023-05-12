package cli

import (
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"github.com/spf13/cobra"
)

func CmdQueryVestingAccounts() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "vesting-accounts",
		Short: "Query VestingAccounts",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			params := &types.QueryVestingAccountsRequest{
				Pagination: pageReq,
			}

			res, err := queryClient.VestingAccounts(ctx, params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddPaginationFlagsToCmd(cmd, "vesting-accounts")

	return cmd
}
