package config

import (
	"encoding/json"
	"time"

	"github.com/icza/dyno"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

// GenesisModifiers takes genesis as an unmarshaled "any" value and, modifies it and returns the "any" back.
type GenesisModifiers func(gen any) (any, error)

// modifyGenesis is a helper function that chains multiple genesisModifiers funcs in order to make
// chain configuration easier and a lot cleaner.
func modifyGenesis(mods ...GenesisModifiers) func(cc ibc.ChainConfig, genbz []byte) ([]byte, error) {
	return func(cc ibc.ChainConfig, genbz []byte) ([]byte, error) {
		var gen any
		err := json.Unmarshal(genbz, &gen)
		if err != nil {
			return nil, err
		}

		for _, mod := range mods {
			gen, err = mod(gen)
			if err != nil {
				return nil, err
			}
		}

		return json.Marshal(gen)
	}
}

// modifyGenesisSetVotingPeriod sets the governance module voting period to the given duration.
func modifyGenesisSetVotingPeriod(period time.Duration) GenesisModifiers {
	return func(gen any) (any, error) {
		err := dyno.Set(gen, VotingPeriod.String(), "app_state", "gov", "voting_params", "voting_period")
		return gen, err
	}
}

// modifyGenesisICAModule sets the params of ICA module.
func modifyGenesisICAModule(enabled bool, allowMsgs []string) GenesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"host_enabled":   enabled,
			"allow_messages": allowMsgs,
		}
		err := dyno.Set(gen, v, "app_state", "interchainaccounts", "host_genesis_state", "params")
		return gen, err
	}
}

// modifyGenesisICQModule sets the params of ICQ module.
func modifyGenesisICQModule(enabled bool, allowQueries []string) GenesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"host_enabled":  enabled,
			"allow_queries": allowQueries,
		}
		err := dyno.Set(gen, v, "app_state", "interchainquery", "params")
		return gen, err
	}
}
