package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgRequestWithdraw_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgRequestWithdraw
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgRequestWithdraw{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "empty RiskProfile",
			msg: MsgRequestWithdraw{
				Creator: sample.AccAddressStr(),
			},
			err: ErrDepositInvalidRiskProfile,
		}, {
			name: "invalid risk profile",
			msg: MsgRequestWithdraw{
				Creator:     sample.AccAddressStr(),
				RiskProfile: "XYZ",
			},
			err: ErrDepositInvalidRiskProfile,
		},
		{
			name: "invalid vault profile",
			msg: MsgRequestWithdraw{
				Creator:     sample.AccAddressStr(),
				RiskProfile: "HIGH",
				VaultID:     "xyz",
				Coin:        sdk.NewCoin("QSR", sdk.NewInt(1000)),
			},
			err: ErrInvalidVaultId,
		},
		{
			name: "invalid risk profile",
			msg: MsgRequestWithdraw{
				Creator:     sample.AccAddressStr(),
				RiskProfile: "HIGH",
				VaultID:     "orion",
				Coin:        sdk.NewCoin("QSR", sdk.NewInt(1000)),
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
