package cli

import (
	"context"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdShowFeeData() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-fee-data",
		Short: "shows FeeData",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetFeeDataRequest{}

			res, err := queryClient.FeeData(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
