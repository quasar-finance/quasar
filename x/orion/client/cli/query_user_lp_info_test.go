package cli_test

import (
	"fmt"
	"testing"

	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	"github.com/stretchr/testify/require"
	tmcli "github.com/tendermint/tendermint/libs/cli"
	"google.golang.org/grpc/status"

	"github.com/abag/quasarnode/testutil/network"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/client/cli"
	"github.com/abag/quasarnode/x/orion/types"
)

func networkWithUserLPInfoObjects(t *testing.T) (*network.Network, types.UserLPInfo) {
	t.Helper()
	cfg := network.DefaultConfig()
	state := types.GenesisState{}
	require.NoError(t, cfg.Codec.UnmarshalJSON(cfg.GenesisState[types.ModuleName], &state))

	userLPInfo := &types.UserLPInfo{}
	nullify.Fill(&userLPInfo)
	state.UserLPInfo = userLPInfo
	buf, err := cfg.Codec.MarshalJSON(&state)
	require.NoError(t, err)
	cfg.GenesisState[types.ModuleName] = buf
	return network.New(t, cfg), *state.UserLPInfo
}

func TestShowUserLPInfo(t *testing.T) {
	net, obj := networkWithUserLPInfoObjects(t)

	ctx := net.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	for _, tc := range []struct {
		desc string
		args []string
		err  error
		obj  types.UserLPInfo
	}{
		{
			desc: "get",
			args: common,
			obj:  obj,
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			var args []string
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdShowUserLPInfo(), args)
			if tc.err != nil {
				stat, ok := status.FromError(tc.err)
				require.True(t, ok)
				require.ErrorIs(t, stat.Err(), tc.err)
			} else {
				require.NoError(t, err)
				var resp types.QueryGetUserLPInfoResponse
				require.NoError(t, net.Config.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.NotNil(t, resp.UserLPInfo)
				require.Equal(t,
					nullify.Fill(&tc.obj),
					nullify.Fill(&resp.UserLPInfo),
				)
			}
		})
	}
}
