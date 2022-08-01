package cli_test

import (
	"fmt"
	"testing"

	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	"github.com/stretchr/testify/require"
	tmcli "github.com/tendermint/tendermint/libs/cli"
	"google.golang.org/grpc/status"

	"github.com/quasarlabs/quasarnode/testutil/network"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/x/qoracle/client/cli"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func networkWithPoolRankingObjects(t *testing.T) (*network.Network, types.PoolRanking) {
	t.Helper()
	cfg := network.DefaultConfig()
	state := types.GenesisState{}
	require.NoError(t, cfg.Codec.UnmarshalJSON(cfg.GenesisState[types.ModuleName], &state))

	poolRanking := &types.PoolRanking{}
	nullify.Fill(&poolRanking)
	state.PoolRanking = poolRanking
	buf, err := cfg.Codec.MarshalJSON(&state)
	require.NoError(t, err)
	cfg.GenesisState[types.ModuleName] = buf
	return network.New(t, cfg), *state.PoolRanking
}

func TestShowPoolRanking(t *testing.T) {
	net, obj := networkWithPoolRankingObjects(t)

	ctx := net.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	for _, tc := range []struct {
		desc string
		args []string
		err  error
		obj  types.PoolRanking
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
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdShowPoolRanking(), args)
			if tc.err != nil {
				stat, ok := status.FromError(tc.err)
				require.True(t, ok)
				require.ErrorIs(t, stat.Err(), tc.err)
			} else {
				require.NoError(t, err)
				var resp types.QueryGetPoolRankingResponse
				require.NoError(t, net.Config.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.NotNil(t, resp.PoolRanking)
				require.Equal(t,
					nullify.Fill(&tc.obj),
					nullify.Fill(&resp.PoolRanking),
				)
			}
		})
	}
}
