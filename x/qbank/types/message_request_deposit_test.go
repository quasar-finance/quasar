package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgRequestDeposit_ValidateBasic(t *testing.T) {
	tests := []struct {
		name   string
		msg    MsgRequestDeposit
		err    error
		errMsg string
	}{
		{
			name: "invalid address",
			msg: MsgRequestDeposit{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "invalid risk profile",
			msg: MsgRequestDeposit{
				Creator:     sample.AccAddressStr(),
				RiskProfile: "FOO",
			},
			errMsg: "invalid deposit msg risk profile",
		}, {
			name: "valid address",
			msg: MsgRequestDeposit{
				Creator:     sample.AccAddressStr(),
				RiskProfile: "HIGH",
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if err != nil {
				if tt.err != nil {
					require.ErrorIs(t, err, tt.err)
					return
				}
				if tt.errMsg != "" {
					require.Equal(t, err.Error(), tt.errMsg)
					return
				}
			}
			require.NoError(t, err)
		})
	}
}
