package cli

import (
	"strconv"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdGetAllDepsoitInfos() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "get-all-depsoit-infos [vault-id]",
		Short: "Query GetAllDepsoitInfos",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqVaultID := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetAllDepsoitInfosRequest{

				VaultID: reqVaultID,
			}

			res, err := queryClient.GetAllDepsoitInfos(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
