package cli_test

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"

	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	"github.com/quasarlabs/quasarnode/testutil/network"
	"github.com/quasarlabs/quasarnode/x/epochs/client/cli"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
	tmcli "github.com/cometbft/cometbft/libs/cli"
)

func setupNetwork(t *testing.T) *network.Network {
	t.Helper()
	network := network.New(t)
	_, err := network.WaitForHeight(1)
	require.NoError(t, err)

	return network
}

func TestGetCmdCurrentEpoch(t *testing.T) {
	network := setupNetwork(t)

	clientCtx := network.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	testCases := []struct {
		name       string
		identifier string
		args       []string
		expectErr  bool
		resp       types.QueryCurrentEpochResponse
	}{
		{
			"query minutely epoch number",
			"minute",
			common,
			false,
			types.QueryCurrentEpochResponse{
				CurrentEpoch: int64(1),
			},
		},
		{
			"query daily epoch number",
			"day",
			common,
			false,
			types.QueryCurrentEpochResponse{
				CurrentEpoch: int64(1),
			},
		},
		{
			"query weekly epoch number",
			"week",
			common,
			false,
			types.QueryCurrentEpochResponse{
				CurrentEpoch: int64(1),
			},
		},
		{
			"query unavailable epoch number",
			"unavailable",
			common,
			true,
			types.QueryCurrentEpochResponse{},
		},
	}

	for _, tc := range testCases {
		tc := tc

		t.Run(tc.name, func(t *testing.T) {
			cmd := cli.GetCmdCurrentEpoch()
			args := []string{
				tc.identifier,
			}
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, args)
			if tc.expectErr {
				require.Error(t, err)
			} else {
				require.NoError(t, err, out.String())

				var actualResp types.QueryCurrentEpochResponse
				err := clientCtx.Codec.UnmarshalJSON(out.Bytes(), &actualResp)
				require.NoError(t, err)
				require.Equal(t, tc.resp, actualResp)
			}
		})
	}
}

func TestGetCmdEpochsInfos(t *testing.T) {
	var err error
	network := setupNetwork(t)
	clientCtx := network.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	cmd := cli.GetCmdEpochsInfos()

	out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, common)
	require.NoError(t, err, out.String())

	var resp types.QueryEpochsInfoResponse
	err = clientCtx.Codec.UnmarshalJSON(out.Bytes(), &resp)
	require.NoError(t, err)
	require.Equal(t, 4, len(resp.Epochs))
}
