package cli

import (
	"fmt"
	// "strings"

	"github.com/spf13/cobra"

	"github.com/cosmos/cosmos-sdk/client"
	// "github.com/cosmos/cosmos-sdk/client/flags"
	// sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/abag/quasarnode/x/qbank/types"
)

// GetQueryCmd returns the cli query commands for this module
func GetQueryCmd(queryRoute string) *cobra.Command {
	// Group qbank queries under a subcommand
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("Querying commands for the %s module", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(CmdQueryParams())
	cmd.AddCommand(CmdListDeposit())
	cmd.AddCommand(CmdShowDeposit())
	cmd.AddCommand(CmdUserDenomDeposit())

	cmd.AddCommand(CmdListWithdraw())
	cmd.AddCommand(CmdShowWithdraw())
	cmd.AddCommand(CmdShowFeeData())
	cmd.AddCommand(CmdUserDeposit())

	cmd.AddCommand(CmdUserDenomLockupDeposit())

	cmd.AddCommand(CmdUserDenomEpochLockupDeposit())

	cmd.AddCommand(CmdUserWithdraw())

	cmd.AddCommand(CmdUserDenomWithdraw())

	cmd.AddCommand(CmdUserClaimRewards())

	// this line is used by starport scaffolding # 1

	return cmd
}
