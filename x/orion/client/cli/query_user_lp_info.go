package cli

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdShowUserLPInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-user-lp-info",
		Short: "shows UserLPInfo",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetUserLPInfoRequest{}

			res, err := queryClient.UserLPInfo(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
