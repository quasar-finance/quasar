package types

import (
	"testing"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgRequestWithdrawAll_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgRequestWithdrawAll
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgRequestWithdrawAll{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address and vault",
			msg: MsgRequestWithdrawAll{
				Creator: sample.AccAddressStr(),
				VaultID: "orion",
			},
		},
		{
			name: "invalid vault",
			msg: MsgRequestWithdrawAll{
				Creator: sample.AccAddressStr(),
				VaultID: "xyz",
			},
			err: ErrInvalidVaultId,
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
