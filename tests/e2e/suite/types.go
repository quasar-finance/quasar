package suite

import "github.com/strangelove-ventures/interchaintest/v4/ibc"

type Accounts struct {
	Authority                                                                             ibc.Wallet
	Owner                                                                                 ibc.Wallet
	NewOwner                                                                              ibc.Wallet
	MasterMinter                                                                          ibc.Wallet
	BondTest, BondTest1, BondTest2, BondTest3, BondTest4, BondTest5, BondTest6, BondTest7 ibc.Wallet
}
