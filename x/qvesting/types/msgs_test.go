package types

import (
	errorsmod "cosmossdk.io/errors"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"testing"

	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgCreateVestingAccount_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgCreateVestingAccount
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreateVestingAccount{
				FromAddress: "invalid_address",
			},
			err: fmt.Errorf("decoding bech32 failed: invalid separator index -1"),
		},
		{
			name: "valid coin",
			msg: MsgCreateVestingAccount{
				FromAddress: sample.AccAddress().String(),
				ToAddress:   sample.AccAddress().String(),
				Amount:      sdk.NewCoins(),
				StartTime:   100000,
				EndTime:     110000,
			},
			err: errorsmod.Wrap(sdkerrors.ErrInvalidCoins, sdk.NewCoins().String()),
		},
		{
			name: "invalid time",
			msg: MsgCreateVestingAccount{
				FromAddress: sample.AccAddress().String(),
				ToAddress:   sample.AccAddress().String(),
				Amount:      sdk.NewCoins(sdk.NewInt64Coin("uqsr", 100000)),
				StartTime:   110000,
				EndTime:     100000,
			},
			err: errorsmod.Wrap(sdkerrors.ErrInvalidRequest, "invalid start time higher than end time"),
		},
		{
			name: "all valid",
			msg: MsgCreateVestingAccount{
				FromAddress: sample.AccAddress().String(),
				ToAddress:   sample.AccAddress().String(),
				Amount:      sdk.NewCoins(sdk.NewInt64Coin("uqsr", 100000)),
				StartTime:   100000,
				EndTime:     110000,
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.Equal(t, err.Error(), tt.err.Error())
				return
			}
			require.NoError(t, err)
		})
	}
}
