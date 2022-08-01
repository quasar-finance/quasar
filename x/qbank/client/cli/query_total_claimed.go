package cli

import (
	"strconv"

	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdTotalClaimed() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "total-claimed [user-acc] [vault-id]",
		Short: "Query TotalClaimed",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqUserAcc := args[0]
			reqVaultID := args[1]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryTotalClaimedRequest{
				UserAcc: reqUserAcc,
				VaultID: reqVaultID,
			}

			res, err := queryClient.TotalClaimed(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
