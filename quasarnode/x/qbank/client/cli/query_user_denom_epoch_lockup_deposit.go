package cli

import (
	"strconv"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cast"
	"github.com/spf13/cobra"
	"strings"
)

var _ = strconv.Itoa(0)

func CmdUserDenomEpochLockupDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "user-denom-epoch-lockup-deposit [user-acc] [denom] [epoch-day] [lockup-type]",
		Short: "Query userDenomEpochLockupDeposit",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqUserAcc := args[0]
			reqDenom := args[1]
			reqCastEpochDay := strings.Split(args[2], listSeparator)
			reqEpochDay := make([]uint64, len(reqCastEpochDay))
			for i, arg := range reqCastEpochDay {
				value, err := cast.ToUint64E(arg)
				if err != nil {
					return err
				}
				reqEpochDay[i] = value
			}
			reqLockupType := args[3]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryUserDenomEpochLockupDepositRequest{

				UserAcc:    reqUserAcc,
				Denom:      reqDenom,
				EpochDay:   reqEpochDay,
				LockupType: reqLockupType,
			}

			res, err := queryClient.UserDenomEpochLockupDeposit(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
