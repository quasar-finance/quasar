package config

import (
	"github.com/strangelove-ventures/ibctest/v5/ibc"
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
		Denom:          "uqsr",
		GasPrices:      "0.00uqsr",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		ModifyGenesis: modifyGenesis(
			modifyGenesisSetVotingPeriod(VotingPeriod),
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
		Denom:          "uatom",
		GasPrices:      "0.00uatom",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
	}

	OsmosisChain = ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "osmosis",
		ChainID: "osmosis",
		Images: []ibc.DockerImage{
			{
				Repository: "ghcr.io/quasar-finance/osmosis",
				Version:    "v12.0.0-icq",
			},
		},
		Bin:            "osmosisd",
		Bech32Prefix:   "osmo",
		Denom:          "uosmo",
		GasPrices:      "0.00uosmo",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		ModifyGenesis: modifyGenesis(
			modifyGenesisICAModule(
				true,
				[]string{
					"/ibc.applications.transfer.v1.MsgTransfer",
					"/osmosis.gamm.poolmodels.balancer.v1beta1.MsgCreateBalancerPool",
					"/osmosis.gamm.v1beta1.MsgJoinPool",
					"/osmosis.gamm.v1beta1.MsgExitPool",
					"/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn",
					"/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut",
					"/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut",
					"/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn",
					"/osmosis.lockup.MsgLockTokens",
					"/osmosis.lockup.MsgBeginUnlocking",
				},
			),
		),
	}
)
