package cli_test

import (
	"fmt"
	"testing"

	"github.com/gogo/protobuf/proto"
	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/testutil/network"
	"github.com/abag/quasarnode/x/epochs/client/cli"
	"github.com/abag/quasarnode/x/epochs/types"
	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	tmcli "github.com/tendermint/tendermint/libs/cli"
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
		respType   proto.Message
	}{
		{
			"query daily epoch number",
			"day",
			common,
			false,
			&types.QueryCurrentEpochResponse{
				CurrentEpoch: int64(1),
			},
		},
		{
			"query weekly epoch number",
			"week",
			common,
			false,
			&types.QueryCurrentEpochResponse{
				CurrentEpoch: int64(0),
			},
		},
		{
			"query unavailable epoch number",
			"unavailable",
			common,
			true,
			&types.QueryCurrentEpochResponse{},
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
				require.NoError(t, clientCtx.Codec.UnmarshalJSON(out.Bytes(), tc.respType), out.String())
			}
		})
	}
}

func TestGetCmdEpochsInfos(t *testing.T) {
	network := setupNetwork(t)

	clientCtx := network.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	testCases := []struct {
		name      string
		args      []string
		expectErr bool
		respType  proto.Message
	}{
		{
			"query default genesis epoch infos",
			common,
			false,
			&types.QueryEpochsInfoResponse{
				Epochs: types.DefaultGenesis().Epochs,
			},
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			cmd := cli.GetCmdEpochsInfos()
			args := tc.args
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
