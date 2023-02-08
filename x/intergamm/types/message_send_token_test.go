package types

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"

	sdkerrors "cosmossdk.io/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgSendToken_ValidateBasic(t *testing.T) {
	sampleZoneId := "osmosis"
	sampleOsmoAddr := "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"
	validCoin := sdk.NewCoin("abc", sdk.NewInt(1000))
	tests := []struct {
		name string
		msg  MsgSendToken
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgSendToken{
				FromAddress: "invalid_address",
				ToZoneId:    sampleZoneId,
				ToAddress:   sampleOsmoAddr,
				Coin:        validCoin,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "missing ToZoneId",
			msg: MsgSendToken{
				FromAddress: sample.AccAddressStr(),
				ToAddress:   sampleOsmoAddr,
				Coin:        validCoin,
			},
			err: ErrInvalidZoneId,
		}, {
			name: "missing ToAddress",
			msg: MsgSendToken{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    sampleZoneId,
				Coin:        validCoin,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "missing amount",
			msg: MsgSendToken{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    sampleZoneId,
				ToAddress:   sampleOsmoAddr,
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "zero amount",
			msg: MsgSendToken{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    sampleZoneId,
				ToAddress:   sampleOsmoAddr,
				Coin:        sdk.NewCoin("abc", sdk.NewInt(0)),
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "valid",
			msg: MsgSendToken{
				FromAddress: sample.AccAddressStr(),
				ToZoneId:    "osmosis",
				ToAddress:   sampleOsmoAddr,
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
