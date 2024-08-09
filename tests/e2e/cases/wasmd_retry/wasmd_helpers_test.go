package wasmd_deposit

import (
	"context"
	"fmt"
	"os"

	sdk "github.com/cosmos/cosmos-sdk/types"
	testsuite "github.com/quasar-finance/quasar/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

// deployPrimitives stores the contract, initiates it and returns the contract address.
func (s *WasmdLPRetryTestSuite) deployPrimitives(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs1, initArgs2, initArgs3 any) (string, string, string) {
	accAddress := acc.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)
	s.PrimitiveStoreID = codeID

	// instantiate the contracts
	res := s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs1)
	s.Require().NotEmpty(res.Address)
	lpStrategyContractAddress1 := res.Address

	// create channels for all the instantiated contracts address 1
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress1),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress1),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)

	res = s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs2)
	s.Require().NotEmpty(res.Address)
	lpStrategyContractAddress2 := res.Address

	// create channels for all the instantiated contracts address 2
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress2),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress2),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)

	res = s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs3)
	s.Require().NotEmpty(res.Address)
	lpStrategyContractAddress3 := res.Address

	// create channels for all the instantiated contracts address 3
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress3),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", lpStrategyContractAddress3),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)
	return lpStrategyContractAddress1, lpStrategyContractAddress2, lpStrategyContractAddress3
}

// deployRewardsContract stores the contract
func (s *WasmdLPRetryTestSuite) deployRewardsContract(ctx context.Context, acc *ibc.Wallet, filePath string) {
	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)
	s.RewardsStoreID = codeID
}

// deployVault stores the contract, initiates it and returns the contract address.
func (s *WasmdLPRetryTestSuite) deployVault(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs any) string {
	accAddress := acc.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)
	s.VaultStoreID = codeID

	res := s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs)
	s.Require().NotEmpty(res.Address)

	return res.Address
}

func (s *WasmdLPRetryTestSuite) setDepositorForContracts(ctx context.Context, acc *ibc.Wallet, initArgs any, lpAddresses []string) {
	s.SetDepositors(ctx, s.Quasar(), lpAddresses[0], acc.KeyName, initArgs)
	s.SetDepositors(ctx, s.Quasar(), lpAddresses[1], acc.KeyName, initArgs)
	s.SetDepositors(ctx, s.Quasar(), lpAddresses[2], acc.KeyName, initArgs)
}

func (s *WasmdLPRetryTestSuite) CreatePools(ctx context.Context) {
	// Read the pool details from os file
	poolBz, err := os.ReadFile(osmosisPool1Path)
	s.Require().NoError(err)
	s.CreatePoolOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz, "")

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool2Path)
	s.Require().NoError(err)
	s.CreatePoolOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz, "")

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool3Path)
	s.Require().NoError(err)
	s.CreatePoolOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz, "")
}

func (s *WasmdLPRetryTestSuite) JoinPools(ctx context.Context, poolIds []string, maxAmountsIn []string, sharesAmountOut []string) {
	// TODO: require len(allTheArgs) is the same
	for i, _ := range poolIds {
		s.JoinPoolOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolIds[i], maxAmountsIn[i], sharesAmountOut[i])
	}
}
