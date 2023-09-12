package suite

import (
	"context"
	"encoding/json"
	"path/filepath"
	"strconv"
	"strings"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"go.uber.org/zap"
)

// StoreContractCode writes the contract into the docker container of chain node, executes the wasm store command
// and returns the code id at the end.
func (s *E2ETestSuite) StoreContractCode(ctx context.Context, chain *cosmos.CosmosChain, keyName string, contract []byte) uint64 {
	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	contractFile := "contract.wasm"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err := fw.WriteFile(ctx, tn.VolumeName, contractFile, contract)
	s.Require().NoError(err, "failed to write contract file")

	txhash, err := tn.ExecTx(ctx, keyName,
		"wasm", "store", filepath.Join(tn.HomeDir(), contractFile),
		"--gas", "20000000",
	)
	s.Require().NoError(err, "failed to store code")

	var resp wasmtypes.MsgStoreCodeResponse
	s.AssertSuccessfulResultTx(ctx, chain, txhash, &resp)

	return resp.CodeID
}

// InstantiateContract instantiates the contract with given codeID on chain. Note that label, admin and funds are optional.
func (s *E2ETestSuite) InstantiateContract(
	ctx context.Context,
	chain *cosmos.CosmosChain,
	keyName string,
	codeID uint64,
	label, admin string,
	funds sdk.Coins,
	args any,
) wasmtypes.MsgInstantiateContractResponse {
	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	s.Require().NoError(err)
	cmds := []string{"wasm", "instantiate",
		strconv.FormatUint(codeID, 10),
		string(argsbz),
		"--gas", "20000000",
	}
	if label != "" {
		cmds = append(cmds, "--label", label)
	}
	if admin != "" {
		cmds = append(cmds, "--admin", admin)
	} else {
		// We must set this explicitly if we don't want an admin
		cmds = append(cmds, "--no-admin")
	}
	if !funds.Empty() {
		cmds = append(cmds, "--amount", funds.String())
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to instantiate contract")

	var resp wasmtypes.MsgInstantiateContractResponse
	s.AssertSuccessfulResultTx(ctx, chain, txhash, &resp)

	return resp
}

// ExecuteContract executes the contract with given contract address on chain. Note that funds are optional.
func (s *E2ETestSuite) ExecuteContract(
	ctx context.Context,
	chain *cosmos.CosmosChain,
	keyName string,
	contractAddr string,
	funds sdk.Coins,
	args any,
	result any,
) {
	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	s.Require().NoError(err)

	cmds := []string{"wasm", "execute",
		contractAddr,
		string(argsbz),
		"--gas", "20000000",
	}
	if !funds.Empty() {
		cmds = append(cmds, "--amount", funds.String())
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to execute contract")

	var resp wasmtypes.MsgExecuteContractResponse
	s.AssertSuccessfulResultTx(ctx, chain, txhash, &resp)

	if result != nil {
		s.Require().NoError(json.Unmarshal(resp.Data, result), "failed to unmarshal result")
	}
}

func (s *E2ETestSuite) ExecuteContractQuery(ctx context.Context, chain *cosmos.CosmosChain, contractAddr string, args any) []byte {
	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	s.Require().NoError(err)

	cmds := []string{"wasm", "contract-state", "smart",
		contractAddr,
		string(argsbz),
		"--output", "json",
	}

	res, _, err := tn.ExecQuery(ctx, cmds...)
	s.Require().NoError(err, "failed to execute query : "+strings.Join(cmds, " "))

	return res
}

func (s *E2ETestSuite) CreatePoolOnOsmosis(ctx context.Context, chain *cosmos.CosmosChain, keyName string, poolBytes []byte, poolType string) {
	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	cmds := []string{"gamm", "create-pool",
		"--gas", "20000000",
	}

	poolFile := "sample.json"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err := fw.WriteFile(ctx, tn.VolumeName, poolFile, poolBytes)
	s.Require().NoError(err, "failed to write pool file")

	cmds = append(cmds, "--pool-file", filepath.Join(tn.HomeDir(), poolFile))

	if len(poolType) != 0 {
		cmds = append(cmds, "--pool-type", poolType)
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to create pool")

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}

func (s *E2ETestSuite) SwapTokenOnOsmosis(ctx context.Context, chain *cosmos.CosmosChain, keyName string, tokenIn string, tokenOutMinAmount string, flagSwapRouteDenoms string, flagSwapRoutePoolIds string) {
	tn := GetFullNode(chain)

	cmds := []string{
		"gamm", "swap-exact-amount-in",
		tokenIn,
		tokenOutMinAmount,
		"--swap-route-denoms", flagSwapRouteDenoms,
		"--swap-route-pool-ids", flagSwapRoutePoolIds,
		"--gas", "20000000",
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to swap token")

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}

func (s *E2ETestSuite) JoinPoolOnOsmosis(ctx context.Context, chain *cosmos.CosmosChain, keyName string, poolId string, maxAmountsIn string, shareAmountOut string) {
	tn := GetFullNode(chain)

	cmds := []string{
		"gamm", "join-pool",
		"--gas", "20000000", "--pool-id", poolId,
		"--max-amounts-in", maxAmountsIn,
		"--share-amount-out", shareAmountOut,
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to join pool")

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}

func (s *E2ETestSuite) CreateStableswapPoolOnOsmosis(ctx context.Context, chain *cosmos.CosmosChain, keyName string, poolBytes []byte) {
	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	cmds := []string{"gamm", "create-pool",
		"--gas", "20000000",
	}

	poolFile := "sample.json"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err := fw.WriteFile(ctx, tn.VolumeName, poolFile, poolBytes)
	s.Require().NoError(err, "failed to write pool file")

	cmds = append(cmds, "--pool-file", filepath.Join(tn.HomeDir(), poolFile), "--pool-type", "stableswap")

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to create pool")

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}

func (s *E2ETestSuite) SendTokensToOneAddress(ctx context.Context, chain *cosmos.CosmosChain, fromAddress, toAddress ibc.Wallet, amount string) {
	tn := GetFullNode(chain)

	cmds := []string{"bank", "send",
		fromAddress.Address, toAddress.Address,
		amount,
	}

	txhash, err := tn.ExecTx(ctx, fromAddress.KeyName, cmds...)
	s.Require().NoError(err, "failed to send tokens")

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}

func (s *E2ETestSuite) SetDepositors(ctx context.Context, chain *cosmos.CosmosChain, primitiveAddress, keyName string, args any) {
	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	s.Require().NoError(err)

	cmds := []string{"wasm", "execute",
		primitiveAddress,
		string(argsbz),
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmds...)
	s.Require().NoError(err, "failed to set depositor for tokens"+primitiveAddress)

	s.AssertSuccessfulResultTx(ctx, chain, txhash, nil)
}
