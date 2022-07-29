package cli

import (
	"context"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdListPoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list-pool-position",
		Short: "list all PoolPosition",
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryAllPoolPositionRequest{
				Pagination: pageReq,
			}

			res, err := queryClient.PoolPositionAll(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddPaginationFlagsToCmd(cmd, cmd.Use)
	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdShowPoolPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-pool-position [pool-id]",
		Short: "shows a PoolPosition",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			argPoolId := args[0]

			params := &types.QueryGetPoolPositionRequest{
				PoolId: argPoolId,
			}

			res, err := queryClient.PoolPosition(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
