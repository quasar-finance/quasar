package cli_test

import (
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"strconv"
	"testing"
	"time"

	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	"github.com/stretchr/testify/require"
	tmcli "github.com/tendermint/tendermint/libs/cli"
	"google.golang.org/grpc/status"

	"github.com/quasarlabs/quasarnode/testutil/network"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/x/orion/client/cli"
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

func networkWithRewardCollectionObjects(t *testing.T) (*network.Network, types.RewardCollection) {
	t.Helper()
	cfg := network.DefaultConfig()
	state := types.GenesisState{}
	require.NoError(t, cfg.Codec.UnmarshalJSON(cfg.GenesisState[types.ModuleName], &state))

	rewardCollection := &types.RewardCollection{
		TimeCollected: time.Now().UTC(),
		Coins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
	}
	nullify.Fill(&rewardCollection)
	state.RewardCollection = rewardCollection
	buf, err := cfg.Codec.MarshalJSON(&state)
	require.NoError(t, err)
	cfg.GenesisState[types.ModuleName] = buf
	return network.New(t, cfg), *state.RewardCollection
}

func TestShowRewardCollection(t *testing.T) {
	net, obj := networkWithRewardCollectionObjects(t)

	ctx := net.Validators[0].ClientCtx
	common := []string{
		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
	}
	for _, tc := range []struct {
		desc       string
		idEpochDay uint64
		args       []string
		err        error
		obj        types.RewardCollection
	}{
		{
			desc:       "get",
			idEpochDay: 9, // TODO replace with correct value stored in obj
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
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdShowRewardCollection(), args)
			if tc.err != nil {
				stat, ok := status.FromError(tc.err)
				require.True(t, ok)
				require.ErrorIs(t, stat.Err(), tc.err)
			} else {
				require.NoError(t, err)
				var resp types.QueryGetRewardCollectionResponse
				require.NoError(t, net.Config.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.NotNil(t, resp.RewardCollection)
				require.Equal(t,
					nullify.Fill(&tc.obj),
					nullify.Fill(&resp.RewardCollection),
				)
			}
		})
	}
}
