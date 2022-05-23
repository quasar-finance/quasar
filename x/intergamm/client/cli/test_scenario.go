//go:build !prod

package cli

import (
	"bytes"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"
	"time"

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

func runScenario(cmd *cobra.Command, scenario string) error {
	var err error

	clientCtx, err := client.GetClientTxContext(cmd)
	if err != nil {
		return err
	}

	var writer bytes.Buffer
	clientCtx.OutputFormat = "json"
	clientCtx.Output = &writer

	txf := tx.NewFactoryCLI(clientCtx, cmd.Flags()).WithTxConfig(clientCtx.TxConfig).WithAccountRetriever(clientCtx.AccountRetriever)

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
}

func queryIca(cmd *cobra.Command, owner string, connectionId string) (string, error) {
	clientCtx, err := client.GetClientTxContext(cmd)
	if err != nil {
		return "", err
	}

	queryClient := types.NewQueryClient(clientCtx)

	params := &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	}
	res, err := queryClient.InterchainAccountFromAddress(cmd.Context(), params)
	if err != nil {
		return "", err
	}

	return res.InterchainAccountAddress, nil
}

func ensureIcaReady(cmd *cobra.Command) (string, error) {
	retries := 0
	maxRetries := 30
	for {
		res, err := queryIca(cmd, "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "connection-0")
		if err == nil && res != "" {
			return res, nil
		}

		if retries >= maxRetries {
			break
		}

		retries++
		time.Sleep(1 * time.Second)
	}

	return "", errors.New("could not retrieve ICA")
}

func runCommand(cmd *cobra.Command, args []string) error {
	var err error

	scenario, err := cmd.Flags().GetString("scenario")
	if err != nil {
		return err
	}
	if scenario == "" {
		return errors.New("must pass in a scenario using the --scenario flag")
	}

	switch scenario {
	case "setup":
		err = runScenario(cmd, "registerIca")
		if err != nil {
			return err
		}

		_, err = ensureIcaReady(cmd)
		if err != nil {
			return err
		}

	default:
		err = runScenario(cmd, scenario)
		if err != nil {
			return err
		}
	}

	return nil
}

func CmdTestScenario() *cobra.Command {
	cmd := &cobra.Command{
		Use:     "test",
		Short:   "Test an intergamm scenario",
		Long:    `Must provide scenario name (--scenario)`,
		Example: `testBasics`,
		Args:    cobra.ExactArgs(0),
		RunE:    runCommand,
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
