package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgAddDenomPriceMapping_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgAddDenomPriceMapping
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgAddDenomPriceMapping{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgAddDenomPriceMapping{
				Creator: sample.AccAddress().String(),
				Mapping: DenomPriceMapping{
					Denom:       "uatom",
					OracleDenom: "ATOM",
					Multiplier:  sdk.NewDecWithPrec(1, 6), // 1e-6
				},
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
