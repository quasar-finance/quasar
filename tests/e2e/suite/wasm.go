package suite

import (
	"context"
	"encoding/json"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"path/filepath"
	"strconv"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
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
		"--gas", "auto",
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
	msg string, result any,
) {
	tn := GetFullNode(chain)

	cmds := []string{"wasm", "execute",
		contractAddr,
		msg,
		"--gas", "auto",
		"--fees", "1000000uqsr",
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

func (s *E2ETestSuite) CreatePoolsOnOsmosis(ctx context.Context, chain *cosmos.CosmosChain, keyName string, poolBytes []byte) {
	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	cmds := []string{"gamm", "create-pool",
		"--gas", "20000000",
		"--fees", "1000000uosmo",
	}

	poolFile := "sample.json"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err := fw.WriteFile(ctx, tn.VolumeName, poolFile, poolBytes)
	s.Require().NoError(err, "failed to write pool file")

	cmds = append(cmds, "--pool-file", filepath.Join(tn.HomeDir(), poolFile))

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
