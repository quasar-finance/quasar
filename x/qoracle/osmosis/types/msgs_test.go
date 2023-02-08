package types

import (
	"testing"

	sdkerrors "cosmossdk.io/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgUpdateChainParams_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgUpdateChainParams
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdateChainParams{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgUpdateChainParams{
				Creator: sample.AccAddress().String(),
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
