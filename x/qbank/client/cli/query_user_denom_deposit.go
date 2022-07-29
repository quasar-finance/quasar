package cli

import (
	"strconv"

	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdUserDenomDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "user-denom-deposit [user-acc]",
		Short: "Query userDenomDeposit",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqUserAcc := args[0]
			reqDenom := args[1]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryUserDenomDepositRequest{

				UserAcc: reqUserAcc,
				Denom:   reqDenom,
			}

			res, err := queryClient.UserDenomDeposit(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
