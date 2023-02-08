package types

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"

	sdkerrors "cosmossdk.io/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgTransmitICATransfer_ValidateBasic(t *testing.T) {
	sampleAmount := sdk.NewCoin("abc", sdk.NewInt(1000))
	tests := []struct {
		name string
		msg  MsgTransmitICATransfer
		err  error
	}{
		{
			name: "invalid ICA owner address",
			msg: MsgTransmitICATransfer{
				IcaOwnerAddress: "invalid_address",
				ToAddress:       sample.AccAddressStr(),
				Coin:            sampleAmount,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "invalid to-address",
			msg: MsgTransmitICATransfer{
				IcaOwnerAddress: sample.AccAddressStr(),
				Coin:            sampleAmount,
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "missing coin",
			msg: MsgTransmitICATransfer{
				IcaOwnerAddress: sample.AccAddressStr(),
				ToAddress:       sample.AccAddressStr(),
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "invalid coin",
			msg: MsgTransmitICATransfer{
				IcaOwnerAddress: sample.AccAddressStr(),
				ToAddress:       sample.AccAddressStr(),
				Coin:            sdk.NewCoin("abc", sdk.NewInt(0)),
			},
			err: sdkerrors.ErrInvalidCoins,
		}, {
			name: "valid",
			msg: MsgTransmitICATransfer{
				IcaOwnerAddress: sample.AccAddressStr(),
				ToAddress:       sample.AccAddressStr(),
				Coin:            sampleAmount,
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
