package types

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgSendTokenToICA_ValidateBasic(t *testing.T) {
	sampleZoneId := "osmosis"
	validCoin := sdk.NewCoin("abc", sdk.NewInt(1000))
	tests := []struct {
		name string
		msg  MsgSendTokenToICA
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgSendTokenToICA{
				FromAddress: "invalid_address",
				ToZoneId:    sampleZoneId,
				Coin:        validCoin,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "missing ToZoneId",
			msg: MsgSendTokenToICA{
				FromAddress: sample.AccAddressStr(),
				Coin:        validCoin,
			},
			err: ErrInvalidZoneId,
		}, {
			name: "missing amount",
			msg: MsgSendTokenToICA{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    sampleZoneId,
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "zero amount",
			msg: MsgSendTokenToICA{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    sampleZoneId,
				Coin:        sdk.NewCoin("abc", sdk.NewInt(0)),
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "valid",
			msg: MsgSendTokenToICA{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    "osmosis",
				Coin:        validCoin,
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
