package cmd

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	appparams "github.com/quasar-finance/quasar/app/params"
)

func InitTestConfig() {
	// Set prefixes
	accountPubKeyPrefix := appparams.Bech32PrefixAccAddr + "pub"
	validatorAddressPrefix := appparams.Bech32PrefixAccAddr + "valoper"
	validatorPubKeyPrefix := appparams.Bech32PrefixAccAddr + "valoperpub"
	consNodeAddressPrefix := appparams.Bech32PrefixAccAddr + "valcons"
	consNodePubKeyPrefix := appparams.Bech32PrefixAccAddr + "valconspub"

	// Set and seal config
	config := sdk.GetConfig()
	config.SetBech32PrefixForAccount(appparams.Bech32PrefixAccAddr, accountPubKeyPrefix)
	config.SetBech32PrefixForValidator(validatorAddressPrefix, validatorPubKeyPrefix)
	config.SetBech32PrefixForConsensusNode(consNodeAddressPrefix, consNodePubKeyPrefix)
}
