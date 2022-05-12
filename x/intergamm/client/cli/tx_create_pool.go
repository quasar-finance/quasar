package cli

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammpooltypes "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/spf13/cobra"
	pflag "github.com/spf13/pflag"
)

var _ = strconv.Itoa(0)

func CmdCreatePool() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "create-pool",
		Short: "Broadcast message createPool",
		Long:  `Must provide path to data JSON file (--data-file) containing the input data`,
		Example: `Sample pool JSON file contents:
{
	"connection_id": "connection-0",
	"timeout_timestamp": "42",
	"weights": "4uatom,4osmo,2uakt",
	"initial_deposit": "100uatom,5osmo,20uakt",
	"swap_fee": "0.01",
	"exit_fee": "0.01",
	"future_governor": "168h"
}
`,
		Args: cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			txf := tx.NewFactoryCLI(clientCtx, cmd.Flags()).WithTxConfig(clientCtx.TxConfig).WithAccountRetriever(clientCtx.AccountRetriever)
			msg, err := parseFlags(clientCtx, txf, cmd.Flags())
			if err != nil {
				return err
			}

			return tx.GenerateOrBroadcastTxWithFactory(clientCtx, txf, msg)
		},
	}

	fs := pflag.NewFlagSet("", pflag.ContinueOnError)
	fs.String("data-file", "", "JSON data file path")

	cmd.Flags().AddFlagSet(fs)
	flags.AddTxFlagsToCmd(cmd)

	err := cmd.MarkFlagRequired("data-file")
	if err != nil {
		panic(err)
	}

	return cmd
}

func parseFlags(clientCtx client.Context, txf tx.Factory, fs *pflag.FlagSet) (sdk.Msg, error) {
	var err error

	dataFile, err := fs.GetString("data-file")
	if err != nil || dataFile == "" {
		return nil, fmt.Errorf("must pass in json data file using the --%s flag", "data-file")
	}

	contents, err := ioutil.ReadFile(dataFile)
	if err != nil {
		return nil, err
	}

	inputData, err := parseInputData(contents)
	if err != nil {
		return nil, fmt.Errorf("failed to parse input data: %w", err)
	}

	msg, err := inputDataToMessage(clientCtx.GetFromAddress().String(), inputData)
	if err != nil {
		return nil, fmt.Errorf("failed to parse input data: %w", err)
	}
	err = msg.ValidateBasic()
	if err != nil {
		return nil, fmt.Errorf("failed to parse input data: %w", err)
	}

	return msg, nil
}

type smoothWeightChangeParamsInputs struct {
	StartTime         string `json:"start_time"`
	Duration          string `json:"duration"`
	TargetPoolWeights string `json:"target_pool_weights"`
}

type InputData struct {
	ConnectionId             string                         `json:"connection_id"`
	TimeoutTimestamp         string                         `json:"timeout_timestamp"`
	Weights                  string                         `json:"weights"`
	InitialDeposit           string                         `json:"initial_deposit"`
	SwapFee                  string                         `json:"swap_fee"`
	ExitFee                  string                         `json:"exit_fee"`
	FutureGovernor           string                         `json:"future_governor"`
	SmoothWeightChangeParams smoothWeightChangeParamsInputs `json:"lbp_params"`
}

func parseInputData(data []byte) (*InputData, error) {
	inputData := &InputData{}
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.DisallowUnknownFields() // Force

	err := dec.Decode(inputData)
	if err != nil {
		return nil, err
	}

	if inputData.ConnectionId == "" {
		return nil, errors.New("connection_id required")
	}

	if inputData.TimeoutTimestamp == "" {
		return nil, errors.New("connection_id required")
	}

	if inputData.InitialDeposit == "" {
		return nil, errors.New("initial_deposit required")
	}

	if inputData.Weights == "" {
		return nil, errors.New("weights required")
	}

	if inputData.FutureGovernor == "" {
		return nil, errors.New("future_governor required")
	}

	return inputData, nil
}

func inputDataToMessage(sender string, inputData *InputData) (*types.MsgCreatePool, error) {
	var err error

	deposit, err := sdk.ParseCoinsNormalized(inputData.InitialDeposit)
	if err != nil {
		return nil, err
	}

	poolAssetCoins, err := sdk.ParseDecCoins(inputData.Weights)
	if err != nil {
		return nil, err
	}

	if len(deposit) != len(poolAssetCoins) {
		return nil, errors.New("deposit tokens and token weights should have same length")
	}

	swapFee, err := sdk.NewDecFromStr(inputData.SwapFee)
	if err != nil {
		return nil, err
	}

	exitFee, err := sdk.NewDecFromStr(inputData.ExitFee)
	if err != nil {
		return nil, err
	}

	var poolAssets []gammtypes.PoolAsset
	for i := 0; i < len(poolAssetCoins); i++ {
		if poolAssetCoins[i].Denom != deposit[i].Denom {
			return nil, errors.New("deposit tokens and token weights should have same denom order")
		}

		poolAssets = append(poolAssets, gammtypes.PoolAsset{
			Weight: poolAssetCoins[i].Amount.RoundInt(),
			Token:  deposit[i],
		})
	}

	poolParams := &gammpooltypes.PoolParams{
		SwapFee: swapFee,
		ExitFee: exitFee,
	}

	timeout, err := strconv.ParseUint(inputData.TimeoutTimestamp, 10, 64)

	msg := types.NewMsgCreatePool(
		sender,
		inputData.ConnectionId,
		timeout,
		poolParams,
		poolAssets,
		inputData.FutureGovernor,
	)
	err = msg.ValidateBasic()
	if err != nil {
		return nil, err
	}

	return msg, nil
}
