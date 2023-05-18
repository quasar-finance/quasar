package suite

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
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
}

func (c Chains) GetChain(chainName string) (*Chain, bool) {
	for _, ch := range c {
		if ch.Chain.Config().Name == chainName {
			return ch, true
		}
	}
	return &Chain{}, false
}

func (p *Chain) SetContracts(contracts []*Contract) {
	p.contracts = append(p.contracts, contracts...)
}

func (p *Chain) FindContractByType(contractType string) *Contract {
	for _, ct := range p.contracts {
		if ct.contractType == contractType {
			return ct
		}
	}
	return nil
}

func (c *Chain) GetContracts() []*Contract {
	if c.IsWasmEnabled {
		return c.contracts
	}
	return nil
}

func (c *Chain) CreateUserAndFund(suite *suite.Suite, ctx context.Context, amount int64) (*ibc.Wallet, error) {
	user := ibctest.GetAndFundTestUsers(suite.T(), ctx, strings.ReplaceAll(suite.T().Name(), " ", "-"), amount, c.Chain)[0]

	// Wait a few blocks
	err := testutil.WaitForBlocks(ctx, 5, c.Chain)
	if err != nil {
		return nil, err
	}

	return user, nil
}

func (c *Chain) ExecQuery(ctx context.Context, resp any, cmd ...string) error {
	tn := GetFullNode(c.Chain)
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

func (c *Chain) ExecTx(ctx context.Context, keyName string, cmd ...string) (string, error) {
	tn := GetFullNode(c.Chain)
	txhash, err := tn.ExecTx(ctx, keyName, cmd...)
	if err != nil {
		return "", err
	}

	return txhash, nil
}

func (c *Chain) AssertSuccessfulResultTx(ctx context.Context, txhash string, resp proto.Message) error {
	tn := GetFullNode(c.Chain)

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
