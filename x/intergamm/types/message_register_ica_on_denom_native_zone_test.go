package types

import (
	"testing"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgRegisterICAOnDenomNativeZone_ValidateBasic(t *testing.T) {
	validDenom := "abc"
	tests := []struct {
		name string
		msg  MsgRegisterICAOnDenomNativeZone
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgRegisterICAOnDenomNativeZone{
				OwnerAddress: "invalid_address",
				Denom:        validDenom,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "missing denom",
			msg: MsgRegisterICAOnDenomNativeZone{
				OwnerAddress: sample.AccAddress().String(),
			},
			err: ErrInvalidDenom,
		}, {
			name: "invalid denom",
			msg: MsgRegisterICAOnDenomNativeZone{
				OwnerAddress: sample.AccAddress().String(),
				Denom:        "a",
			},
			err: ErrInvalidDenom,
		}, {
			name: "valid",
			msg: MsgRegisterICAOnDenomNativeZone{
				OwnerAddress: sample.AccAddress().String(),
				Denom:        validDenom,
			},
			err: nil,
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
