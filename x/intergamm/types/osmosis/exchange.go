package osmosis

import (
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	"github.com/pkg/errors"
)

type exchangeRequest interface {
	proto.Message

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

func ParseAck[RES exchangeResponse](ack channeltypes.Acknowledgement, expected proto.Message) (RES, error) {
	if ack.GetError() != "" {
		return nil, errors.New(ack.GetError())
	}

	err := proto.Unmarshal(ack.GetResult(), expected)
	if err != nil {
		return nil, errors.Wrap(err, "cannot unmarshall Osmosis acknowledgement")
	}

	switch msg := expected.(type) {
	case RES:
		return msg, nil
	default:
		return nil, errors.New("error while parsing Osmosis acknowledgement")
	}
}
