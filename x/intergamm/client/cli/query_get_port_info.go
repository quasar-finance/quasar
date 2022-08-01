package cli

import (
	"strconv"

	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdGetPortInfo() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "get-port-info [destination-chain-id][port-id]",
		Short: "Query getPortInfo",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqDestinationChainId := args[0]
			reqPortID := args[1]
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryGetPortInfoRequest{

				PortID:             reqPortID,
				DestinationChainID: reqDestinationChainId,
			}

			res, err := queryClient.GetPortInfo(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
