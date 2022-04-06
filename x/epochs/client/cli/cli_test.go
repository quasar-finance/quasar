package cli_test

import (
	"testing"

	"github.com/gogo/protobuf/proto"
	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/x/epochs/client/cli"
	"github.com/abag/quasarnode/x/epochs/types"
	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	"github.com/cosmos/cosmos-sdk/testutil/network"
)

func setupNetwork(t *testing.T) *network.Network {
	cfg := network.DefaultConfig()
	network := network.New(t, cfg)

	_, err := network.WaitForHeight(1)
	require.NoError(t, err)

	return network
}

func TestGetCmdCurrentEpoch(t *testing.T) {
	network := setupNetwork(t)
	defer network.Cleanup()

	val := network.Validators[0]

	testCases := []struct {
		name       string
		identifier string
		expectErr  bool
		respType   proto.Message
	}{
		{
			"query weekly epoch number",
			"weekly",
			false, &types.QueryCurrentEpochResponse{},
		},
		{
			"query unavailable epoch number",
			"unavailable",
			false, &types.QueryCurrentEpochResponse{},
		},
	}

	for _, tc := range testCases {
		tc := tc

		t.Run(tc.name, func(t *testing.T) {
			cmd := cli.GetCmdCurrentEpoch()
			clientCtx := val.ClientCtx

			args := []string{
				tc.identifier,
			}

			out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, args)
			if tc.expectErr {
				require.Error(t, err)
			} else {
				require.NoError(t, err, out.String())
				require.NoError(t, clientCtx.Codec.UnmarshalJSON(out.Bytes(), tc.respType), out.String())
			}
		})
	}
}

func TestGetCmdEpochsInfos(t *testing.T) {
	network := setupNetwork(t)
	defer network.Cleanup()

	val := network.Validators[0]

	testCases := []struct {
		name      string
		expectErr bool
		respType  proto.Message
	}{
		{
			"query epoch infos",
			false, &types.QueryEpochsInfoResponse{},
		},
	}

	for _, tc := range testCases {
		tc := tc

		t.Run(tc.name, func(t *testing.T) {
			cmd := cli.GetCmdCurrentEpoch()
			clientCtx := val.ClientCtx

			args := []string{}

			out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, args)
			if tc.expectErr {
				require.Error(t, err)
			} else {
				require.NoError(t, err, out.String())
				require.NoError(t, clientCtx.Codec.UnmarshalJSON(out.Bytes(), tc.respType), out.String())
			}
		})
	}
}
