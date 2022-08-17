package cli

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/spf13/cobra"
)

var _ = strconv.Itoa(0)

func CmdRequestDeposit() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "request-deposit [vault-id=orion] [sdk.coin = 1000qsr] [Lockupperiod = Days_7/Days_21/Months_1/Months_3] [Reserved = comma separated list of fields]",
		Short: "Broadcast message requestDeposit",
		Args:  cobra.ExactArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			argVaultID := args[0]
			argCoinStr := args[1]
			argLockupType := args[2]
			argReserved := args[3] // comma separated reserved fields
			reservedFields := []string{}
			if argReserved != "" {
				reservedFields = strings.Split(argReserved, ",")
			}

			lockupPeriodInt := types.LockupTypes(types.LockupTypes_value[argLockupType])

			CoinStr, err := sdk.ParseCoinNormalized(argCoinStr)
			if err != nil {
				return fmt.Errorf("invalid coin string")
			}
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			msg := types.NewMsgRequestDeposit(
				clientCtx.GetFromAddress().String(),
				argVaultID,
				CoinStr,
				lockupPeriodInt,
				reservedFields,
			)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
