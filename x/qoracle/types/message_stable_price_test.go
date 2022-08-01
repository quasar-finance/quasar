package types

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgStablePrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgStablePrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgStablePrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid fields",
			msg: MsgStablePrice{
				Creator: sample.AccAddress().String(),
				Denom:   "uatom",
				Price:   "10.10",
			},
		}, {
			name: "invalid price",
			msg: MsgStablePrice{
				Creator: sample.AccAddress().String(),
				Denom:   "uatom",
				Price:   "-10.10",
			},
			err: ErrInvalidStablePrice,
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
