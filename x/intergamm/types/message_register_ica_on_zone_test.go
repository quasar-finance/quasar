package types

import (
	"testing"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgRegisterICAOnZone_ValidateBasic(t *testing.T) {
	sampleZoneId := "osmosis"
	tests := []struct {
		name string
		msg  MsgRegisterICAOnZone
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgRegisterICAOnZone{
				OwnerAddress: "invalid_address",
				ZoneId:       sampleZoneId,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "invalid zone id",
			msg: MsgRegisterICAOnZone{
				OwnerAddress: sample.AccAddress().String(),
			},
			err: ErrInvalidZoneId,
		}, {
			name: "valid",
			msg: MsgRegisterICAOnZone{
				OwnerAddress: sample.AccAddress().String(),
				ZoneId:       sampleZoneId,
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
