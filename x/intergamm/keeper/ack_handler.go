package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

func (k *Keeper) HandleIcaAcknowledgement(
	ctx sdk.Context,
	sequence uint64,
	icaPacket icatypes.InterchainAccountPacketData,
	ack channeltypes.Acknowledgement,
) error {
	msgs, err := icatypes.DeserializeCosmosTx(k.cdc, icaPacket.GetData())
	if err != nil {
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "cannot deserilize packet data")
	}

	if len(msgs) != 1 {
		return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "expected single message in packet")
	}

	msg := msgs[0]
	switch msg := msg.(type) {
	case *gammbalancer.MsgCreateBalancerPool:
		res, err := types.ParseAck[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse](ack, msg)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]{
			Sequence: sequence,
			Error:    err,
			Request:  msg,
			Response: res,
		}
		for _, h := range k.hooksOsmosisMsgCreateBalancerPool {
			h.HandleMsgCreateBalancerPool(ctx, ex)
		}
	}

	return nil
}
