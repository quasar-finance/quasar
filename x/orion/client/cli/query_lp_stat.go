package cli

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdShowLpStat() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-lp-stat",
		Short: "shows LpStat",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetLpStatRequest{}

			res, err := queryClient.LpStat(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
