package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdICAAddressOnDenomNativeZone() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "ica-address-on-denom-native-zone [owner] [denom]",
		Short: "Query the inter-chain address of owner on the native zone of the given denom",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqOwner := args[0]
			reqDenom := args[1]

			clientCtx, err := client.GetClientQueryContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)

			params := &types.QueryICAAddressOnDenomNativeZoneRequest{
				Owner: reqOwner,
				Denom: reqDenom,
			}

			res, err := queryClient.ICAAddressOnDenomNativeZone(cmd.Context(), params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
