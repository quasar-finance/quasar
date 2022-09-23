package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdICAAddressOnZone() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "ica-address-on-zone [owner] [zone-id]",
		Short: "Query the inter-chain address of owner on the given zone",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqOwner := args[0]
			reqZoneId := args[1]

			clientCtx, err := client.GetClientQueryContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryICAAddressOnZoneRequest{
				Owner:  reqOwner,
				ZoneId: reqZoneId,
			}

			res, err := queryClient.ICAAddressOnZone(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
