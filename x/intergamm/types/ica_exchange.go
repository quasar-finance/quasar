package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

type exchangeRequest interface {
	sdk.Msg

	*ibctransfertypes.MsgTransfer | *gammbalancer.MsgCreateBalancerPool | *gammtypes.MsgJoinPool | *gammtypes.MsgExitPool
}

type exchangeResponse interface {
	proto.Message

	*ibctransfertypes.MsgTransferResponse | *gammbalancer.MsgCreateBalancerPoolResponse | *gammtypes.MsgJoinPoolResponse | *gammtypes.MsgExitPoolResponse
}

type AckExchange[REQ exchangeRequest, RES exchangeResponse] struct {
	Sequence uint64
	Error    string
	Request  REQ
	Response RES
}

func (e AckExchange[REQ, RES]) HasError() bool {
	return e.Error != ""
}

type TimeoutExchange[REQ exchangeRequest] struct {
	Sequence uint64
	Request  REQ
}
