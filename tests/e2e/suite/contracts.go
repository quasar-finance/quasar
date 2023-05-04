package suite

import (
	"context"
	"encoding/hex"
	"fmt"
	"go.uber.org/zap"
	"os"
	"path/filepath"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

func StoreContractCode(ctx context.Context, chain *cosmos.CosmosChain, filePath string, acc *ibc.Wallet, s *E2ETestSuiteBuilder) (uint64, error) {
	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	if err != nil {
		return 0, err
	}

	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	contractFile := "contract.wasm"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err = fw.WriteFile(ctx, tn.VolumeName, contractFile, contract)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to write contract file")
	}

	txhash, err := tn.ExecTx(ctx, acc.KeyName,
		"wasm", "store", filepath.Join(tn.HomeDir(), contractFile),
		"--gas", "20000000",
	)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to store code")
	}

	var resp wasmtypes.MsgStoreCodeResponse
	txhashBytes, err := hex.DecodeString(txhash)
	if err != nil {
		return 0, err
	}
	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to find tx result %s", txhash)
	}
	if res.TxResult.Code != 0 {
		return 0, fmt.Errorf("tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)
	}

	// Only unmarshal result if user wants to
	if &resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, &resp)
		return 0, err
	}

	return resp.CodeID, nil
}
