package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdListActiveLps() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list-active-lps",
		Short: "Query list_active_lps",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryListActiveLpsRequest{}

			res, err := queryClient.ListActiveLps(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
