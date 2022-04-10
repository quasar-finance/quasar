package cli

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdShowEpochLPInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-epoch-lp-info",
		Short: "shows EpochLPInfo",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetEpochLPInfoRequest{}

			res, err := queryClient.EpochLPInfo(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
