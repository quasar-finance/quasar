package cli_test

import (
	"fmt"
	"strconv"
	"testing"
	"time"

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

func networkWithLpPositionObjects(t *testing.T) (*network.Network, types.LpPosition) {
	t.Helper()
	cfg := network.DefaultConfig()
	state := types.GenesisState{}
	require.NoError(t, cfg.Codec.UnmarshalJSON(cfg.GenesisState[types.ModuleName], &state))

	lpPosition := &types.LpPosition{
		LpID:                   42,
		LockID:                 24,
		IsActive:               false,
		StartTime:              time.Now().UTC(),
		BondingStartEpochDay:   2,
		BondDuration:           5,
		UnbondingStartEpochDay: 7,
		UnbondingDuration:      8,
		PoolID:                 1,
		Lptoken:                sdk.NewCoin("LPT", sdk.NewInt(100)),
		Coins:                  sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
		Gaugelocks:             nil,
	}
	nullify.Fill(&lpPosition)
	state.LpPosition = lpPosition
	buf, err := cfg.Codec.MarshalJSON(&state)
	require.NoError(t, err)
	cfg.GenesisState[types.ModuleName] = buf
	return network.New(t, cfg), *state.LpPosition
}

func TestShowLpPosition(t *testing.T) {
	net, obj := networkWithLpPositionObjects(t)

	ctx := net.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	for _, tc := range []struct {
		desc   string
		idLpId uint64
		args   []string
		err    error
		obj    types.LpPosition
	}{
		{
			desc:   "get",
			idLpId: obj.LpID,
			args:   common,
			obj:    obj,
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			args := []string{
				strconv.FormatUint(tc.idLpId, 10),
			}
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdShowLpPosition(), args)
			if tc.err != nil {
				stat, ok := status.FromError(tc.err)
				require.True(t, ok)
				require.ErrorIs(t, stat.Err(), tc.err)
			} else {
				require.NoError(t, err)
				var resp types.QueryGetLpPositionResponse
				require.NoError(t, net.Config.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.NotNil(t, resp.LpPosition)
				require.Equal(t,
					nullify.Fill(&tc.obj),
					nullify.Fill(&resp.LpPosition),
				)
			}
		})
	}
}
