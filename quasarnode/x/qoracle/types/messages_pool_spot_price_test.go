package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgCreatePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgCreatePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreatePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgCreatePoolSpotPrice{
				Creator: sample.AccAddress(),
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgUpdatePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgUpdatePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdatePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgUpdatePoolSpotPrice{
				Creator: sample.AccAddress(),
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgDeletePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgDeletePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgDeletePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgDeletePoolSpotPrice{
				Creator: sample.AccAddress(),
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}
