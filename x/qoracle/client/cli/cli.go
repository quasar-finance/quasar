package cli

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/spf13/cobra"

	qbandcli "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/client/cli"
	qosmocli "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/client/cli"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetQueryCmd returns the cli query commands for this module
func GetQueryCmd() *cobra.Command {
	// Group qoracle queries under a subcommand
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("Querying commands for the %s module", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(
		CmdQueryParams(),
		CmdQueryDenomPrices(),
		CmdQueryPools(),
		qbandcli.GetQueryCmd(),
		qosmocli.GetQueryCmd(),
	)

	return cmd
}
