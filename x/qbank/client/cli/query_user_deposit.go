package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdUserDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "user-deposit [user-acc]",
		Short: "Query userDeposit",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqUserAcc := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryUserDepositRequest{

				UserAcc: reqUserAcc,
			}

			res, err := queryClient.UserDeposit(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
