package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdOsmosisPools() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "osmosis-pools",
		Short: "Query OsmosisPools",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryOsmosisPoolsRequest{}

			res, err := queryClient.OsmosisPools(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
