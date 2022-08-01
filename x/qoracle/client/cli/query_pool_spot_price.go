package cli

import (
	"context"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdListPoolSpotPrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list-pool-spot-price",
		Short: "list all PoolSpotPrice",
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryAllPoolSpotPriceRequest{
				Pagination: pageReq,
			}

			res, err := queryClient.PoolSpotPriceAll(context.Background(), params)
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

func CmdShowPoolSpotPrice() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-pool-spot-price [pool-id] [denom-in] [denom-out]",
		Short: "shows a PoolSpotPrice",
		Args:  cobra.ExactArgs(3),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			argPoolId := args[0]
			argDenomIn := args[1]
			argDenomOut := args[2]

			params := &types.QueryGetPoolSpotPriceRequest{
				PoolId:   argPoolId,
				DenomIn:  argDenomIn,
				DenomOut: argDenomOut,
			}

			res, err := queryClient.PoolSpotPrice(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
