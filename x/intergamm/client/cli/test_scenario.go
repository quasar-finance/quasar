//go:build !prod

package cli

import (
	"bytes"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	proto "github.com/gogo/protobuf/proto"
	"github.com/spf13/cobra"
	pflag "github.com/spf13/pflag"
)

var _ = strconv.Itoa(0)

func init() {
	addCommand(CmdTestScenario())
}

func CmdTestScenario() *cobra.Command {
	cmd := &cobra.Command{
		Use:     "test",
		Short:   "Test an intergamm scenario",
		Long:    `Must provide scenario name (--scenario)`,
		Example: `testBasics`,
		Args:    cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) error {
			var err error

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			var writer bytes.Buffer
			clientCtx.OutputFormat = "json"
			clientCtx.Output = &writer

			txf := tx.NewFactoryCLI(clientCtx, cmd.Flags()).WithTxConfig(clientCtx.TxConfig).WithAccountRetriever(clientCtx.AccountRetriever)

			scenario, err := cmd.Flags().GetString("scenario")
			if err != nil {
				return err
			}
			if scenario == "" {
				return errors.New("must pass in a scenario using the --scenario flag")
			}

			msg := &types.MsgTestScenario{
				Creator:  clientCtx.GetFromAddress().String(),
				Scenario: scenario,
			}

			err = tx.GenerateOrBroadcastTxWithFactory(clientCtx, txf, msg)
			if err != nil {
				return err
			}

			var txRes sdk.TxResponse
			err = clientCtx.Codec.UnmarshalJSON(writer.Bytes(), &txRes)
			if err != nil {
				return err
			}

			if txRes.Code == 0 {
				resData, err := hex.DecodeString(txRes.Data)
				if err != nil {
					return err
				}

				txMsgData := &sdk.TxMsgData{}
				err = proto.Unmarshal(resData, txMsgData)
				if err != nil {
					return err
				}

				if len(txMsgData.Data) != 1 {
					return errors.New("only single msg acks are supported")
				}

				msgData := txMsgData.Data[0]
				res := &types.MsgTestScenarioResponse{}
				err = proto.Unmarshal(msgData.GetData(), res)
				if err != nil {
					return err
				}

				fmt.Println(res.GetResult())

				if res.GetExitCode() != 0 {
					return errors.New("test scenario failed")
				}
			} else {
				fmt.Println(writer.String())
			}

			return nil
		},
	}

	fs := pflag.NewFlagSet("", pflag.ContinueOnError)
	fs.String("scenario", "", "scenario")

	cmd.Flags().AddFlagSet(fs)
	flags.AddTxFlagsToCmd(cmd)

	err := cmd.MarkFlagRequired("scenario")
	if err != nil {
		panic(err)
	}

	return cmd
}
