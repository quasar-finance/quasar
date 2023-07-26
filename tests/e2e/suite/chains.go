package suite

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
	"go.uber.org/zap"
	"path/filepath"
	"strings"

	"github.com/gogo/protobuf/proto"
	ibctest "github.com/strangelove-ventures/interchaintest/v4"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

var (
	QuasarChain = ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "quasar",
		ChainID: "quasar",
		Images: []ibc.DockerImage{
			{
				Repository: "quasar",
				Version:    "local",
			},
		},
		Bin:            "quasarnoded",
		Bech32Prefix:   "quasar",
		CoinType:       "118",
		Denom:          "uqsr",
		GasPrices:      "0.01uqsr",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		ModifyGenesis: modifyGenesis(
			modifyGenesisSetVotingPeriod(VotingPeriod),
		),
	}

	OsmosisChain = ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "osmosis",
		ChainID: "osmosis",
		Images: []ibc.DockerImage{
			{
				Repository: "osmosis",
				Version:    "local",
			},
		},
		Bin:            "osmosisd",
		Bech32Prefix:   "osmo",
		Denom:          "uosmo",
		CoinType:       "118",
		GasPrices:      "0.01uosmo",
		GasAdjustment:  1.5,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		ModifyGenesis: modifyGenesis(
			modifyGenesisICAModule(
				true,
				ICAAllowedMessages,
				"icahost",
			),
			modifyGenesisICQModule(
				true,
				ICQAllowedQueries,
				"icqhost",
			),
			modifyMintModule(),
			modifyIncentivesModule(),
			modifyPoolIncentivesModule(),
			modifyEpochsModule(),
		),
	}

	CosmosChain = ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "cosmos",
		ChainID: "cosmos",
		Images: []ibc.DockerImage{
			{
				Repository: "ghcr.io/quasar-finance/gaia",
				Version:    "v7.1.0-router",
			},
		},
		Bin:            "gaiad",
		Bech32Prefix:   "cosmos",
		CoinType:       "118",
		Denom:          "uatom",
		GasPrices:      "0.00uatom",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
	}
)

type Chains []*Chain

type Chain struct {
	Chain         *cosmos.CosmosChain
	IsWasmEnabled bool
	contracts     []*Contract
	ChainAccount  AccountsNew
	TestCases     TestCases
}

func (c Chains) GetChain(chainName string) (*Chain, bool) {
	for _, ch := range c {
		if ch.Chain.Config().Name == chainName {
			return ch, true
		}
	}
	return &Chain{}, false
}

func (p *Chain) ExecuteTests(ctx context.Context) error {
	err := p.TestCases.ExecuteCases(p.Chain, ctx)
	if err != nil {
		return err
	}

	return nil
}

func (p *Chain) SetContracts(contracts []*Contract) error {
	if !p.IsWasmEnabled {
		return fmt.Errorf("chain is not wasm enabled, chain name : %s", p.Chain.Config().Name)
	}
	p.contracts = append(p.contracts, contracts...)
	return nil
}

func (p *Chain) FindContractByLabel(contractLabel string) (*Contract, error) {
	if !p.IsWasmEnabled {
		return nil, fmt.Errorf("chain is not wasm enabled, chain name : %s", p.Chain.Config().Name)
	}

	for _, ct := range p.contracts {
		if ct.label == contractLabel {
			return ct, nil
		}
	}

	return nil, fmt.Errorf("contract type does not exist in chain : %s", p.Chain.Config().Name)
}

func (p *Chain) GetContracts() ([]*Contract, error) {
	if p.IsWasmEnabled {
		return p.contracts, nil
	} else {
		return nil, fmt.Errorf("chain is not wasm enabled, chain name : %s", p.Chain.Config().Name)
	}
}

func (p *Chain) CreateUserAndFund(suite *suite.Suite, ctx context.Context, amount int64) (*ibc.Wallet, error) {
	user := ibctest.GetAndFundTestUsers(suite.T(), ctx, strings.ReplaceAll(suite.T().Name(), " ", "-"), amount, p.Chain)[0]

	// Wait a few blocks
	err := testutil.WaitForBlocks(ctx, 5, p.Chain)
	if err != nil {
		return nil, err
	}

	return user, nil
}

func (p *Chain) ExecQuery(ctx context.Context, resp any, cmd ...string) error {
	tn := GetFullNode(p.Chain)
	stdout, stderr, err := tn.ExecQuery(ctx, cmd...)
	if err != nil {
		return err
	}
	if string(stderr) != "" {
		return fmt.Errorf(string(stderr))
	}

	err = json.Unmarshal(stdout, &resp)
	if err != nil {
		return err
	}

	return nil
}

func (p *Chain) ExecTx(ctx context.Context, cmd []string, keyName, fileName, fileFlag string, fileBytes []byte, amount sdk.Coins, logger *zap.Logger) (string, error) {
	tn := GetFullNode(p.Chain)

	if !amount.Empty() {
		cmd = append(cmd, "--amount", amount.String())
	}

	if fileName != "" && fileBytes != nil {
		fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
		err := fw.WriteFile(ctx, tn.VolumeName, fileName, fileBytes)
		if err != nil {
			return "", fmt.Errorf(err.Error(), "failed to write pool file")
		}

		cmd = append(cmd, fileFlag, filepath.Join(tn.HomeDir(), fileName))
	}

	txhash, err := tn.ExecTx(ctx, keyName, cmd...)
	if err != nil {
		return "", fmt.Errorf(err.Error(), "failed to execute command :", strings.Join(cmd, " "))
	}

	return txhash, nil
}

func (p *Chain) AssertSuccessfulResultTx(ctx context.Context, txhash string, resp proto.Message) error {
	tn := GetFullNode(p.Chain)

	txhashBytes, err := hex.DecodeString(txhash)
	if err != nil {
		return err
	}

	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	if err != nil {
		return fmt.Errorf(err.Error(), "failed to find tx result %s", txhash)
	}
	if res.TxResult.Code != 0 {
		return fmt.Errorf("tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)
	}

	// Only unmarshal result if user wants to
	if resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, resp)
		if err != nil {
			return err
		}
	}

	return nil
}
