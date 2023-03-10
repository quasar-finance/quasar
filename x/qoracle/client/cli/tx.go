package cli

import (
	"fmt"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/spf13/cobra"

	"github.com/cosmos/cosmos-sdk/client"
	qosmocli "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/client/cli"
)

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("%s transactions subcommands", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(qosmocli.GetTxCmd())
	return cmd
}
