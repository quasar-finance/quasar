package types

import (
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	lockuptypes "github.com/osmosis-labs/osmosis/v7/x/lockup/types"
)

type ibcExchangeRequest interface {
	proto.Message

	*ibctransfertypes.FungibleTokenPacketData | *gammbalancer.MsgCreateBalancerPool | *gammtypes.MsgJoinPool | *gammtypes.MsgJoinSwapExternAmountIn | *gammtypes.MsgExitPool | *lockuptypes.MsgLockTokens
}

type ibcExchangeResponse interface {
	proto.Message

	*MsgEmptyIbcResponse | *gammbalancer.MsgCreateBalancerPoolResponse | *gammtypes.MsgJoinPoolResponse | *gammtypes.MsgJoinSwapExternAmountInResponse | *gammtypes.MsgExitPoolResponse | *lockuptypes.MsgLockTokensResponse
}

type AckExchange[REQ ibcExchangeRequest, RES ibcExchangeResponse] struct {
	Sequence uint64
	Error    string
	Request  REQ
	Response RES
}

func (e AckExchange[REQ, RES]) HasError() bool {
	return e.Error != ""
}

type TimeoutExchange[REQ ibcExchangeRequest] struct {
	Sequence uint64
	Request  REQ
}
