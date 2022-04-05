package cli_test

import (
	"fmt"
	"testing"

	"github.com/cosmos/cosmos-sdk/client/flags"
	clitestutil "github.com/cosmos/cosmos-sdk/testutil/cli"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/testutil/network"
	"github.com/abag/quasarnode/x/qoracle/client/cli"
)

func TestCreatePoolSpotPrice(t *testing.T) {
	net := network.New(t)
	val := net.Validators[0]
	ctx := val.ClientCtx

	fields := []string{"1.11"}
	for _, tc := range []struct {
		desc       string
		idPoolId   string
		idDenomIn  string
		idDenomOut string

		args []string
		err  error
		code uint32
	}{
		{
			idPoolId:   "1",
			idDenomIn:  "abc",
			idDenomOut: "cba",

			desc: "valid",
			args: []string{
				fmt.Sprintf("--%s=%s", flags.FlagFrom, val.Address.String()),
				fmt.Sprintf("--%s=true", flags.FlagSkipConfirmation),
				fmt.Sprintf("--%s=%s", flags.FlagBroadcastMode, flags.BroadcastBlock),
				fmt.Sprintf("--%s=%s", flags.FlagFees, sdk.NewCoins(sdk.NewCoin(net.Config.BondDenom, sdk.NewInt(10))).String()),
			},
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			args := []string{
				tc.idPoolId,
				tc.idDenomIn,
				tc.idDenomOut,
			}
			args = append(args, fields...)
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdCreatePoolSpotPrice(), args)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				var resp sdk.TxResponse
				require.NoError(t, ctx.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.Equal(t, tc.code, resp.Code)
			}
		})
	}
}

func TestUpdatePoolSpotPrice(t *testing.T) {
	net := network.New(t)
	val := net.Validators[0]
	ctx := val.ClientCtx

	fields := []string{"1.11"}
	common := []string{
		fmt.Sprintf("--%s=%s", flags.FlagFrom, val.Address.String()),
		fmt.Sprintf("--%s=true", flags.FlagSkipConfirmation),
		fmt.Sprintf("--%s=%s", flags.FlagBroadcastMode, flags.BroadcastBlock),
		fmt.Sprintf("--%s=%s", flags.FlagFees, sdk.NewCoins(sdk.NewCoin(net.Config.BondDenom, sdk.NewInt(10))).String()),
	}
	args := []string{
		"1",
		"abc",
		"cba",
	}
	args = append(args, fields...)
	args = append(args, common...)
	_, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdCreatePoolSpotPrice(), args)
	require.NoError(t, err)

	for _, tc := range []struct {
		desc       string
		idPoolId   string
		idDenomIn  string
		idDenomOut string

		args []string
		code uint32
		err  error
	}{
		{
			desc:       "valid",
			idPoolId:   "1",
			idDenomIn:  "abc",
			idDenomOut: "cba",

			args: common,
		},
		{
			desc:       "key not found",
			idPoolId:   "10",
			idDenomIn:  "xyz",
			idDenomOut: "zyx",

			args: common,
			code: sdkerrors.ErrKeyNotFound.ABCICode(),
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			args := []string{
				tc.idPoolId,
				tc.idDenomIn,
				tc.idDenomOut,
			}
			args = append(args, fields...)
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdUpdatePoolSpotPrice(), args)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				var resp sdk.TxResponse
				require.NoError(t, ctx.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.Equal(t, tc.code, resp.Code)
			}
		})
	}
}

func TestDeletePoolSpotPrice(t *testing.T) {
	net := network.New(t)

	val := net.Validators[0]
	ctx := val.ClientCtx

	fields := []string{"1.11"}
	common := []string{
		fmt.Sprintf("--%s=%s", flags.FlagFrom, val.Address.String()),
		fmt.Sprintf("--%s=true", flags.FlagSkipConfirmation),
		fmt.Sprintf("--%s=%s", flags.FlagBroadcastMode, flags.BroadcastBlock),
		fmt.Sprintf("--%s=%s", flags.FlagFees, sdk.NewCoins(sdk.NewCoin(net.Config.BondDenom, sdk.NewInt(10))).String()),
	}
	args := []string{
		"1",
		"abc",
		"cba",
	}
	args = append(args, fields...)
	args = append(args, common...)
	_, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdCreatePoolSpotPrice(), args)
	require.NoError(t, err)

	for _, tc := range []struct {
		desc       string
		idPoolId   string
		idDenomIn  string
		idDenomOut string

		args []string
		code uint32
		err  error
	}{
		{
			desc:       "valid",
			idPoolId:   "1",
			idDenomIn:  "abc",
			idDenomOut: "cba",

			args: common,
		},
		{
			desc:       "key not found",
			idPoolId:   "10",
			idDenomIn:  "xyz",
			idDenomOut: "zyx",

			args: common,
			code: sdkerrors.ErrKeyNotFound.ABCICode(),
		},
	} {
		tc := tc
		t.Run(tc.desc, func(t *testing.T) {
			args := []string{
				tc.idPoolId,
				tc.idDenomIn,
				tc.idDenomOut,
			}
			args = append(args, tc.args...)
			out, err := clitestutil.ExecTestCLICmd(ctx, cli.CmdDeletePoolSpotPrice(), args)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				var resp sdk.TxResponse
				require.NoError(t, ctx.Codec.UnmarshalJSON(out.Bytes(), &resp))
				require.Equal(t, tc.code, resp.Code)
			}
		})
	}
}
