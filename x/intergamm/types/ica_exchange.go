package types

import (
	gammbalancer "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/osmosis/v9/gamm/types"
	lockuptypes "github.com/abag/quasarnode/osmosis/v9/lockup/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	proto "github.com/gogo/protobuf/proto"
)

type ibcExchangeRequest interface {
	proto.Message

	*ibctransfertypes.FungibleTokenPacketData |
		*ibctransfertypes.MsgTransfer |
		*gammbalancer.MsgCreateBalancerPool |
		*gammtypes.MsgJoinPool |
		*gammtypes.MsgExitPool |
		*gammtypes.MsgJoinSwapExternAmountIn |
		*gammtypes.MsgExitSwapExternAmountOut |
		*gammtypes.MsgJoinSwapShareAmountOut |
		*gammtypes.MsgExitSwapShareAmountIn |
		*lockuptypes.MsgLockTokens |
		*lockuptypes.MsgBeginUnlocking
}

type ibcExchangeResponse interface {
	proto.Message

	*MsgEmptyIbcResponse |
		*ibctransfertypes.MsgTransferResponse |
		*gammbalancer.MsgCreateBalancerPoolResponse |
		*gammtypes.MsgJoinPoolResponse |
		*gammtypes.MsgExitPoolResponse |
		*gammtypes.MsgJoinSwapExternAmountInResponse |
		*gammtypes.MsgExitSwapExternAmountOutResponse |
		*gammtypes.MsgJoinSwapShareAmountOutResponse |
		*gammtypes.MsgExitSwapShareAmountInResponse |
		*lockuptypes.MsgLockTokensResponse |
		*lockuptypes.MsgBeginUnlockingResponse
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
