package cli

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/cosmos/cosmos-sdk/testutil"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func samplePoolFile1() string {
	return `
{
	"address": "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t",
	"id": 1,
	"pool_params": {
		"swapFee": "0.003000000000000000",
		"exitFee": "0.000000000000000000",
		"smoothWeightChangeParams": null
	},
	"future_pool_governor": "24h",
	"total_shares": {
		"denom": "gamm/pool/1",
		"amount": "401669780697469189120477614"
	},
	"pool_assets": [
		{
			"token": {
				"denom": "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
				"amount": "8642970429489"
			},
			"weight": "536870912000000"
		},
		{
			"token": {
				"denom": "uosmo",
				"amount": "25868659405488"
			},
			"weight": "536870912000000"
		}
	],
	"total_weight": "1073741824000000"
}
`
}

func TestParseBalancerPoolFile(t *testing.T) {
	okJSON := testutil.WriteToNewTempFile(t, samplePoolFile1())

	badJSON := testutil.WriteToNewTempFile(t, "bad json")

	// nonexistent json
	_, err := parseBalancerPoolFile("fileDoesNotExist")
	require.Error(t, err)

	// invalid json
	_, err = parseBalancerPoolFile(badJSON.Name())
	require.Error(t, err)

	// ok json
	pool, err := parseBalancerPoolFile(okJSON.Name())
	require.NoError(t, err, "unexpected error")
	require.Equal(t, "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t", pool.GetAddress().String())
	require.Equal(t, uint64(1), pool.GetId())
	require.Equal(t, sdk.MustNewDecFromStr("0.003000000000000000"), pool.PoolParams.SwapFee)
	require.Equal(t, sdk.MustNewDecFromStr("0.000000000000000000"), pool.PoolParams.ExitFee)

	err = okJSON.Close()
	require.NoError(t, err, "unexpected error")
	err = badJSON.Close()
	require.NoError(t, err, "unexpected error")
}
