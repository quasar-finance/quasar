package cli_test

import (
	"fmt"
	"strconv"
	"testing"

	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	tmcli "github.com/tendermint/tendermint/libs/cli"
	"google.golang.org/grpc/status"

	"github.com/abag/quasarnode/testutil/network"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/client/cli"
	"github.com/abag/quasarnode/x/orion/types"
)

func networkWithEpochLPInfoObjects(t *testing.T) (*network.Network, types.EpochLPInfo) {
	t.Helper()
	cfg := network.DefaultConfig()
	state := types.GenesisState{}
	require.NoError(t, cfg.Codec.UnmarshalJSON(cfg.GenesisState[types.ModuleName], &state))

	epochLPInfo := &types.EpochLPInfo{
		EpochDay: 42,
		TotalLps: 2,
		TotalTVL: sdk.NewCoin("abc", sdk.NewInt(100)),
	}
	nullify.Fill(&epochLPInfo)
	state.EpochLPInfo = epochLPInfo
	buf, err := cfg.Codec.MarshalJSON(&state)
	require.NoError(t, err)
	cfg.GenesisState[types.ModuleName] = buf
	return network.New(t, cfg), *state.EpochLPInfo
}

func TestShowEpochLPInfo(t *testing.T) {
	net, obj := networkWithEpochLPInfoObjects(t)

	ctx := net.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	for _, tc := range []struct {
		desc       string
		idEpochDay uint64
		args       []string
		err        error
		obj        types.EpochLPInfo
	}{
		{
			desc:       "get",
			idEpochDay: obj.EpochDay,
			args:       common,
			obj:        obj,
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			args := []string{
				strconv.FormatUint(tc.idEpochDay, 10),
			}
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdShowEpochLPInfo(), args)
			if tc.err != nil {
				stat, ok := status.FromError(tc.err)
				require.True(t, ok)
				require.ErrorIs(t, stat.Err(), tc.err)
			} else {
				require.NoError(t, err)
				var resp types.QueryGetEpochLPInfoResponse
				require.NoError(t, net.Config.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.NotNil(t, resp.EpochLPInfo)
				require.Equal(t,
					nullify.Fill(&tc.obj),
					nullify.Fill(&resp.EpochLPInfo),
				)
			}
		})
	}
}
