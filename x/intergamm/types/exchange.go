package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	"github.com/pkg/errors"
)

type exchangeRequest interface {
	sdk.Msg

	*gammbalancer.MsgCreateBalancerPool
}

type exchangeResponse interface {
	proto.Message

	*gammbalancer.MsgCreateBalancerPoolResponse
}

type Exchange[REQ exchangeRequest, RES exchangeResponse] struct {
	Sequence uint64
	Error    error
	Request  REQ
	Response RES
}

func (e Exchange[REQ, RES]) HasError() bool {
	return e.Error != nil
}

func mappings[RES exchangeResponse](requestType string) (RES, error) {
	switch requestType {
	case sdk.MsgTypeURL(&gammbalancer.MsgCreateBalancerPool{}):
		return &gammbalancer.MsgCreateBalancerPoolResponse{}, nil
	default:
		return nil, errors.New("unsupported acknowledgement mapping")
	}
}

// Spec doc:
// https://github.com/cosmos/ibc-go/blob/main/docs/apps/interchain-accounts/auth-modules.md#onacknowledgementpacket
func ParseAck[REQ exchangeRequest, RES exchangeResponse](ack channeltypes.Acknowledgement, req REQ) (RES, error) {
	if ack.GetError() != "" {
		return nil, errors.New(ack.GetError())
	}

	txMsgData := &sdk.TxMsgData{}
	if err := proto.Unmarshal(ack.GetResult(), txMsgData); err != nil {
		return nil, errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
	}

	switch len(txMsgData.Data) {
	case 0:
		// see documentation below for SDK 0.46.x or greater
		return nil, errors.New("unsupported operation")
	default:
		if len(txMsgData.Data) != 1 {
			return nil, errors.New("only single msg acks are supported")
		}

		msgData := txMsgData.Data[0]
		msgType := msgData.GetMsgType()

		if msgType != sdk.MsgTypeURL(req) {
			return nil, errors.New("ack response does not match request")
		}

		dst, err := mappings(msgType)
		if err != nil {
			return nil, errors.Wrap(err, "unknown ack mapping")
		}

		err = proto.Unmarshal(msgData.Data, dst)
		if err != nil {
			return nil, errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
		}

		return dst, nil
	}
}
