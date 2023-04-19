package cli

import (
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"github.com/spf13/cobra"
)

func CmdQuerySpendableBalances() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "spendable-balances [address]",
		Short: "Query SpendableBalances",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqAddress := args[0]

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

			params := &types.QuerySpendableBalancesRequest{
				Address:    reqAddress,
				Pagination: pageReq,
			}

			res, err := queryClient.SpendableBalances(ctx, params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)
	flags.AddPaginationFlagsToCmd(cmd, "spendable-balances")

	return cmd
}
