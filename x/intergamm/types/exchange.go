package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/pkg/errors"
)

type exchangeRequest interface {
	sdk.Msg

	*gammbalancer.MsgCreateBalancerPool | *gammtypes.MsgJoinPool
}

type exchangeResponse interface {
	proto.Message

	*gammbalancer.MsgCreateBalancerPoolResponse | *gammtypes.MsgJoinPoolResponse
}

type Exchange[REQ exchangeRequest, RES exchangeResponse] struct {
	Sequence uint64
	Error    string
	Request  REQ
	Response RES
}

func (e Exchange[REQ, RES]) HasError() bool {
	return e.Error != ""
}

// Spec doc:
// https://github.com/cosmos/ibc-go/blob/main/docs/apps/interchain-accounts/auth-modules.md#onacknowledgementpacket
func ParseAck(ack channeltypes.Acknowledgement, request sdk.Msg, response proto.Message) error {
	if ack.GetError() != "" {
		return nil
	}

	txMsgData := &sdk.TxMsgData{}
	if err := proto.Unmarshal(ack.GetResult(), txMsgData); err != nil {
		return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
	}

	switch len(txMsgData.Data) {
	case 0:
		// see documentation below for SDK 0.46.x or greater
		return errors.New("currently unsupported operation")
	default:
		if len(txMsgData.Data) != 1 {
			return errors.New("only single msg acks are supported")
		}

		msgData := txMsgData.Data[0]
		msgType := msgData.GetMsgType()

		if msgType != sdk.MsgTypeURL(request) {
			return errors.New("ack response does not match request")
		}

		err := proto.Unmarshal(msgData.Data, response)
		if err != nil {
			return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
		}

		return nil
	}
}
