package types

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgClaimRewards_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgClaimRewards
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgClaimRewards{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "invalid vault",
			msg: MsgClaimRewards{
				Creator: sample.AccAddressStr(),
				VaultID: "xyz",
			},
			err: ErrInvalidVaultId,
		},
		{
			name: "valid address and vault",
			msg: MsgClaimRewards{
				Creator: sample.AccAddressStr(),
				VaultID: "orion",
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
