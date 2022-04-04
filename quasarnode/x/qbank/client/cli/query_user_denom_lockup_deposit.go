package cli

import (
	"strconv"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdUserDenomLockupDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "user-denom-lockup-deposit [user-acc] [denom] [lockup-type]",
		Short: "Query userDenomLockupDeposit",
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqUserAcc := args[0]
			reqDenom := args[1]
			reqLockupType := args[2]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryUserDenomLockupDepositRequest{

				UserAcc:    reqUserAcc,
				Denom:      reqDenom,
				LockupType: reqLockupType,
			}

			res, err := queryClient.UserDenomLockupDeposit(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
