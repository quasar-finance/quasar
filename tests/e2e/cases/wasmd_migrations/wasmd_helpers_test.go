package wasmd

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"os"
	"strconv"
	"sync"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	transfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

// deployPrimitives stores the contract, initiates it and returns the contract address.
func (s *WasmdTestSuite) deployPrimitives(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs1, initArgs2, initArgs3 any) {
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
	s.LpStrategyContractAddress1 = res.Address

	// create channels for all the instantiated contracts address 1
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress1),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress1),
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
	s.LpStrategyContractAddress2 = res.Address

	// create channels for all the instantiated contracts address 2
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress2),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress2),
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
	s.LpStrategyContractAddress3 = res.Address

	// create channels for all the instantiated contracts address 3
	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress3),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testsuite.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress3),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)
}

// deployRewardsContract stores the contract
func (s *WasmdTestSuite) deployRewardsContract(ctx context.Context, acc *ibc.Wallet, filePath string) {
	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)
	s.RewardsStoreID = codeID
}

// deployVault stores the contract, initiates it and returns the contract address.
func (s *WasmdTestSuite) deployVault(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs any) string {
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

func (s *WasmdTestSuite) setDepositorForContracts(ctx context.Context, acc *ibc.Wallet, initArgs any) {
	s.SetDepositors(ctx, s.Quasar(), s.LpStrategyContractAddress1, acc.KeyName, initArgs)
	s.SetDepositors(ctx, s.Quasar(), s.LpStrategyContractAddress2, acc.KeyName, initArgs)
	s.SetDepositors(ctx, s.Quasar(), s.LpStrategyContractAddress3, acc.KeyName, initArgs)
}

func (s *WasmdTestSuite) CreatePools(ctx context.Context) {
	// Read the pool details from os file
	poolBz, err := os.ReadFile(osmosisPool1Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz)

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool2Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz)

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool3Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz)
}

// ibcDenomFromChannel returns ibc denom according to the given channel port, id and denom
// this function generates the ibc denom for the main direction as an example if there is a channel from
// chain1 <-> chain2 knowing that chain1 has a denom named denom1 this function will return the ibc denom of denom1 in chain2.
func ibcDenomFromChannel(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.PortId, ch.ChannelId, baseDenom)).IBCDenom()
}

// ibcDenomFromChannelCounterparty does same as ibcDenomFromChannel but in reverse so it generates
// the ibc denom of denom2 from chain2 (counterparty chain) in chain1
func ibcDenomFromChannelCounterparty(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.Counterparty.PortId, ch.Counterparty.ChannelId, baseDenom)).IBCDenom()
}

func printWorker(cs <-chan error, done chan<- bool, cc chan<- bool) {
	for i := range cs {
		fmt.Println("printing from output in all test cases:", i)
	}

	done <- true
	cc <- true
}

func (s *WasmdTestSuite) monitorWorker(wg *sync.WaitGroup, cs chan error) {
	wg.Wait()
	close(cs)
}

func (s *WasmdTestSuite) VerifyBond(ctx context.Context, address string, expectedShares int64, expectedDeviation float64, oc chan error, wg *sync.WaitGroup, t *testing.T, testIndex int) {
	defer wg.Done()
	for i := 0; i < 10; i++ {
		var data testsuite.ContractBalanceData
		balanceBytes := s.ExecuteContractQuery(
			ctx,
			s.Quasar(),
			s.BasicVaultContractAddress,
			map[string]any{
				"balance": map[string]any{
					"address": address,
				},
			},
		)

		err := json.Unmarshal(balanceBytes, &data)
		if err != nil && i == 9 {
			oc <- fmt.Errorf(err.Error(), "got it in test case index %d", testIndex)
		}

		balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
		if err != nil && i == 9 {
			oc <- fmt.Errorf(err.Error(), "got it in test case index %d", testIndex)
		}

		if int64(float64(expectedShares)*(1-expectedDeviation)) <= balance && balance <= int64(float64(expectedShares)*(1+expectedDeviation)) {
			oc <- nil
			break
		}

		t.Log("Wait for quasar to clear cache and then check bond success")
		err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
		if err != nil && i == 9 {
			oc <- fmt.Errorf(err.Error(), "got it in test case index %d", testIndex)
		}
	}
}

func (s *WasmdTestSuite) VerifyUnbond(ctx context.Context, address string, expectedNumberOfUnbonds int64, UnbondAmount string, oc chan error, wg *sync.WaitGroup, t *testing.T, testIndex int) {
	defer wg.Done()
	for i := 1; i < 10; i++ {
		var pendingUnbondsData testsuite.PendingUnbondsData
		pendingUnbondsBytes := s.ExecuteContractQuery(
			ctx,
			s.Quasar(),
			s.BasicVaultContractAddress,
			map[string]any{
				"pending_unbonds": map[string]any{
					"address": address,
				},
			},
		)

		err := json.Unmarshal(pendingUnbondsBytes, &pendingUnbondsData)
		if err != nil && i == 9 {
			oc <- fmt.Errorf(err.Error(), "got it in test case index %d", testIndex)
		}

		// verify if the unbonded amount and expected number of unbonds matches their respective conditions or not
		if expectedNumberOfUnbonds == int64(len(pendingUnbondsData.Data.PendingUnbonds)) && UnbondAmount == pendingUnbondsData.Data.PendingUnbonds[expectedNumberOfUnbonds-1].Shares {
			oc <- nil
			break
		}

		t.Log("Wait for quasar to clear cache and then check unbond success")
		err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
		if err != nil && i == 9 {
			oc <- fmt.Errorf(err.Error(), "got it in test case index %d", testIndex)
		}
	}
}
