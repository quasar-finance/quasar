package suite

import (
	"context"
	"encoding/json"
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
		"--gas", "auto",
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
	msg, result any,
) {
	tn := GetFullNode(chain)

	msgbz, err := json.Marshal(msg)
	s.Require().NoError(err)
	cmds := []string{"wasm", "execute",
		contractAddr,
		string(msgbz),
		"--gas", "auto",
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
