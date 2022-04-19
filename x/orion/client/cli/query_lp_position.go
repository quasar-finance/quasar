package cli

import (
	"context"
	"strconv"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

func CmdShowLpPosition() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "show-lp-position  <lpid>",
		Short: "shows lpPosition",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)
			lpid, err := strconv.ParseUint(args[0], 10, 64)
			if err != nil {
				return err
			}
			params := &types.QueryGetLpPositionRequest{LpId: lpid}

			res, err := queryClient.LpPosition(context.Background(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
