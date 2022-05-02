package pf_test

import (
	"testing"

	transfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	"github.com/strangelove-ventures/ibc-test-framework/ibc"
	"github.com/strangelove-ventures/ibc-test-framework/ibctest"
	"github.com/stretchr/testify/require"
)

func TestIBCTransfer(t *testing.T) {
	cf := ibctest.NewBuiltinChainFactory(
		[]ibctest.BuiltinChainFactoryEntry{
			{Name: "gaia", Version: "latest", ChainID: "cosmoshub-1004", NumValidators: 1, NumFullNodes: 1},
			{Name: "osmosis", Version: "latest", ChainID: "osmosis-1001", NumValidators: 1, NumFullNodes: 1},
		},
	)
	rf := ibctest.NewBuiltinRelayerFactory(ibc.CosmosRly)

	ctx, home, pool, network, err := ibctest.SetupTestRun(t)
	require.NoErrorf(t, err, "failed to set up test run")

	srcChain, dstChain, err := cf.Pair(t.Name())
	require.NoError(t, err, "failed to get chain pair")

	// startup both chains and relayer
	// creates wallets in the relayer for src and dst chain
	// funds relayer src and dst wallets on respective chain in genesis
	// creates a user account on the src chain (separate fullnode)
	// funds user account on src chain in genesis
	_, channels, srcUser, dstUser, err := ibctest.StartChainsAndRelayerFromFactory(t, ctx, pool, network, home, srcChain, dstChain, rf, nil)
	require.NoError(t, err, "failed to StartChainsAndRelayerFromFactory")

	// will test a user sending an ibc transfer from the src chain to the dst chain
	// denom will be src chain native denom
	testDenomSrc := srcChain.Config().Denom

	// query initial balance of user wallet for src chain native denom on the src chain
	srcInitialBalance, err := srcChain.GetBalance(ctx, srcUser.SrcChainAddress, testDenomSrc)
	require.NoErrorf(t, err, "failed to get balance from source chain %s", srcChain.Config().Name)

	// get ibc denom for test denom on dst chain
	denomTrace := transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(channels[0].Counterparty.PortID, channels[0].Counterparty.ChannelID, testDenomSrc))
	dstIbcDenom := denomTrace.IBCDenom()

	// query initial balance of user wallet for src chain native denom on the dst chain
	// don't care about error here, account does not exist on destination chain
	dstInitialBalance, _ := dstChain.GetBalance(ctx, srcUser.DstChainAddress, dstIbcDenom)

	t.Logf("Initial source balance: %d", srcInitialBalance)
	t.Logf("Initial dest balance: %d", dstInitialBalance)

	// test coin, address is recipient of ibc transfer on dst chain
	testCoinSrc := ibc.WalletAmount{
		Address: srcUser.DstChainAddress,
		Denom:   testDenomSrc,
		Amount:  1000000,
	}

	// send ibc transfer from the user wallet using its fullnode
	// timeout is nil so that it will use the default timeout
	srcTxHash, err := srcChain.SendIBCTransfer(ctx, channels[0].ChannelID, srcUser.KeyName, testCoinSrc, nil)
	require.NoError(t, err, "failed to send ibc transfer")

	// wait for both chains to produce 20 blocks
	require.NoError(t, ibctest.WaitForBlocks(srcChain, dstChain, 20), "failed to wait for blocks")

	// fetch ibc transfer tx
	srcTx, err := srcChain.GetTransaction(ctx, srcTxHash)
	require.NoError(t, err, "failed to get ibc transaction")

	t.Logf("Transaction: %v", srcTx)

	// query final balance of src user wallet for src chain native denom on the src chain
	srcFinalBalance, err := srcChain.GetBalance(ctx, srcUser.SrcChainAddress, testDenomSrc)
	require.NoError(t, err, "failed to get balance from source chain")

	// query final balance of src user wallet for src chain native denom on the dst chain
	dstFinalBalance, err := dstChain.GetBalance(ctx, srcUser.DstChainAddress, dstIbcDenom)
	require.NoError(t, err, "failed to get balance from dest chain")

	totalFees := srcChain.GetGasFeesInNativeDenom(srcTx.GasWanted)
	expectedDifference := testCoinSrc.Amount + totalFees

	require.Equal(t, srcFinalBalance, srcInitialBalance-expectedDifference)
	require.Equal(t, dstFinalBalance, dstInitialBalance+testCoinSrc.Amount)

	// Now relay from dst chain to src chain using dst user wallet

	// will test a user sending an ibc transfer from the dst chain to the src chain
	// denom will be dst chain native denom
	testDenomDst := dstChain.Config().Denom

	// query initial balance of dst user wallet for dst chain native denom on the dst chain
	dstInitialBalance, err = dstChain.GetBalance(ctx, dstUser.DstChainAddress, testDenomDst)
	require.NoError(t, err, "failed to get balance from dest chain")

	// get ibc denom for test denom on src chain
	srcDenomTrace := transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(channels[0].PortID, channels[0].ChannelID, testDenomDst))
	srcIbcDenom := srcDenomTrace.IBCDenom()

	// query initial balance of user wallet for src chain native denom on the dst chain
	// don't care about error here, account does not exist on destination chain
	srcInitialBalance, _ = srcChain.GetBalance(ctx, dstUser.SrcChainAddress, srcIbcDenom)

	t.Logf("Initial balance on src chain: %d", srcInitialBalance)
	t.Logf("Initial balance on dst chain: %d", dstInitialBalance)

	// test coin, address is recipient of ibc transfer on src chain
	testCoinDst := ibc.WalletAmount{
		Address: dstUser.SrcChainAddress,
		Denom:   testDenomDst,
		Amount:  1000000,
	}

	// send ibc transfer from the dst user wallet using its fullnode
	// timeout is nil so that it will use the default timeout
	dstTxHash, err := dstChain.SendIBCTransfer(ctx, channels[0].Counterparty.ChannelID, dstUser.KeyName, testCoinDst, nil)
	require.NoError(t, err, "failed to send ibc transfer")

	// wait for both chains to produce 20 blocks
	require.NoError(t, ibctest.WaitForBlocks(srcChain, dstChain, 20), "failed to wait for blocks")

	// fetch ibc transfer tx
	dstTx, err := dstChain.GetTransaction(ctx, dstTxHash)
	require.NoError(t, err, "failed to get transaction")

	t.Logf("Transaction: %v", dstTx)

	// query final balance of dst user wallet for dst chain native denom on the dst chain
	dstFinalBalance, err = dstChain.GetBalance(ctx, dstUser.DstChainAddress, testDenomDst)
	require.NoError(t, err, "failed to get dest balance")

	// query final balance of dst user wallet for dst chain native denom on the src chain
	srcFinalBalance, err = srcChain.GetBalance(ctx, dstUser.SrcChainAddress, srcIbcDenom)
	require.NoError(t, err, "failed to get source balance")

	totalFeesDst := dstChain.GetGasFeesInNativeDenom(dstTx.GasWanted)
	expectedDifference = testCoinDst.Amount + totalFeesDst

	require.Equal(t, dstInitialBalance-expectedDifference, dstFinalBalance)
	require.Equal(t, srcInitialBalance+testCoinDst.Amount, srcFinalBalance)
}
