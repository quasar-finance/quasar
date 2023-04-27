package suite

import (
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
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
